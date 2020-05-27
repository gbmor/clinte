use std::env;
use std::fs;
use std::process;

use chrono::prelude::*;

use crate::error;
use crate::user;

fn create_tmp_file<'a>() -> Result<String, std::io::Error> {
    let the_time = Utc::now().to_rfc3339();
    let file_name = format!("/tmp/clinte_ed_{}_{}", *user::NAME, the_time);
    match fs::write(&file_name, "") {
        Ok(_) => Ok(file_name),
        Err(err) => Err(err),
    }
}

pub fn call(body: &str) -> String {
    // If they don't have $EDITOR set, just default to nano
    // instead of assuming vim or emacs.
    let editor = match env::var("EDITOR") {
        Ok(ed) => {
            if &ed == "" {
                "nano".into()
            } else {
                ed
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

    error::helper(
        process::Command::new(editor)
            .arg(&tmp_loc)
            .stdin(process::Stdio::inherit())
            .stdout(process::Stdio::inherit())
            .output(),
        "Couldn't call editor",
    );

    let body = error::helper(
        fs::read_to_string(&tmp_loc),
        "Couldn't read message from disk",
    );
    error::helper(fs::remove_file(tmp_loc), "Couldn't remove temporary file");
    body
}
