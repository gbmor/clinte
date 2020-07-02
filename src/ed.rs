use std::fs;

#[cfg(not(test))]
use std::env;
#[cfg(not(test))]
use std::process;

use chrono::prelude::*;

#[cfg(not(test))]
use crate::error;

use crate::user;

// Creates a temporary file to call $EDITOR on. Returns the
// path to the file on success.
fn create_tmp_file() -> Result<String, std::io::Error> {
    let the_time = Utc::now().to_rfc3339();
    let file_name = format!("/tmp/clinte_ed_{}_{}", *user::NAME, the_time);
    match fs::write(&file_name, "") {
        Ok(_) => Ok(file_name),
        Err(err) => Err(err),
    }
}

// This calls $EDITOR and pre-fills it with the provided text.
#[cfg(not(test))]
pub fn call(body: &str) -> String {
    // If they don't have $EDITOR set, just default to nano
    // instead of assuming vim or emacs.
    let editor = match env::var("EDITOR") {
        Ok(ed) => {
            if &ed == "" {
                "nano".trim().to_string()
            } else {
                ed.trim().to_string()
            }
        }
        Err(_) => {
            log::warn!("Couldn't get value of $EDITOR, defaulting to nano");
            "nano".into()
        }
    };

    let tmp_loc = error::helper(create_tmp_file(), "Couldn't create tempfile");

    error::helper(
        fs::write(&tmp_loc, body),
        "Couldn't populate tempfile with message",
    );

    // Check if $EDITOR contains flags. Change the way we call $EDITOR if so,
    // because otherwise it will explode.
    if editor.contains(" ") {
        let ed_split = editor.split_whitespace().collect::<Vec<&str>>();
        let mut args = vec![];
        ed_split.iter().enumerate().for_each(|(i, e)| {
            if i == 0 {
                return;
            }
            args.push(e.to_string());
        });
        args.push(tmp_loc.clone());
        error::helper(
            process::Command::new(ed_split[0])
                .args(&args)
                .stdin(process::Stdio::inherit())
                .stdout(process::Stdio::inherit())
                .output(),
            "Couldn't call editor",
        );
    } else {
        error::helper(
            process::Command::new(editor)
                .arg(&tmp_loc)
                .stdin(process::Stdio::inherit())
                .stdout(process::Stdio::inherit())
                .output(),
            "Couldn't call editor",
        );
    }

    let body = error::helper(
        fs::read_to_string(&tmp_loc),
        "Couldn't read message from disk",
    );
    error::helper(fs::remove_file(tmp_loc), "Couldn't remove temporary file");
    body
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tempfile() {
        let name = create_tmp_file();
        assert!(name.is_ok());
        let name = name.unwrap();
        fs::remove_file(name).unwrap();
    }
}
