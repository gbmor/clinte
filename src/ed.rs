use std::env;
use std::fs;
use std::process;

use chrono::prelude::*;

use crate::error;
use crate::user;

lazy_static! {
    static ref VAR: String = match env::var("EDITOR") {
        Ok(ed) => {
            if &ed == "" {
                "nano".into()
            } else {
                ed
            }
        }
        Err(err) => {
            log::warn!("{:?}", err);
            "nano".into()
        }
    };
}

fn create_tmp_file<'a>() -> Result<String, &'a str> {
    let the_time = Utc::now().to_rfc3339();
    let file_name = format!("/tmp/clinte_ed_{}_{}", *user::NAME, the_time);
    match fs::write(&file_name, "") {
        Ok(_) => Ok(file_name),
        Err(err) => {
            log::warn!("{:?}", err);
            Err("Unable to create temp file")
        }
    }
}

pub fn call() -> String {
    let tmp_loc = error::helper(create_tmp_file());

    error::helper(
        process::Command::new(VAR.clone())
            .arg(tmp_loc.clone())
            .stdin(process::Stdio::inherit())
            .stdout(process::Stdio::inherit())
            .output(),
    );

    let body = error::helper(fs::read_to_string(tmp_loc.clone()));
    error::helper(fs::remove_file(tmp_loc));
    body
}
