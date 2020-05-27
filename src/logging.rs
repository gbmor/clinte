use std::fs::OpenOptions;

use simplelog::*;

use crate::conf;
use crate::error;
use crate::user;

pub fn checked_init() {
    let logfile = format!("/tmp/clinte_{}.log", *user::NAME);

    if let Err(e) = init(&logfile) {
        log::error!("Couldn't initialize logging. Exiting.");
        if *conf::DEBUG {
            log::error!("--> {}", e);
        }
        std::process::exit(1);
    }
}

fn init(path: &str) -> error::Result<()> {
    let logfile = OpenOptions::new().append(true).create(true).open(path)?;
    WriteLogger::init(LevelFilter::Info, Config::default(), logfile)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    #[test]
    fn init_logs() {
        let file = "clinte_test.log";
        let blank = " ".bytes().collect::<Vec<u8>>();
        fs::write(&file, &blank).unwrap();
        init("clinte_test.log").unwrap();

        log::info!("TEST LOG MESSAGE");
        let logfile = fs::read_to_string(&file).unwrap();
        assert!(logfile.contains("TEST LOG MESSAGE"));

        fs::remove_file("clinte_test.log").unwrap();
    }
}
