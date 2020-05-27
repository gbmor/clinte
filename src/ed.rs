use std::env;
use std::fs;
use std::process;

use chrono::prelude::*;

use crate::conf;
use crate::error;
use crate::user;

fn create_tmp_file<'a>() -> Result<String, &'a str> {
    let the_time = Utc::now().to_rfc3339();
    let file_name = format!("/tmp/clinte_ed_{}_{}", *user::NAME, the_time);
    match fs::write(&file_name, "") {
        Ok(_) => Ok(file_name),
        Err(err) => {
            log::warn!("Couldn't create tempfile");
            if *conf::DEBUG {
                log::warn!("--> {:?}", err);
            }
            Err("Unable to create temp file")
        }
    }
}

pub fn call() -> String {
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

    let tmp_loc = error::helper(create_tmp_file());

    error::helper(
        process::Command::new(editor)
            .arg(&tmp_loc)
            .stdin(process::Stdio::inherit())
            .stdout(process::Stdio::inherit())
            .output(),
    );

    let body = error::helper(fs::read_to_string(&tmp_loc));
    error::helper(fs::remove_file(tmp_loc));
    body
}
