use std::fs::OpenOptions;

use simplelog::*;

use crate::error;
use crate::user;

pub fn init() -> error::Result<()> {
    let file = format!("/tmp/clinte_{}.log", *user::NAME);
    let logfile = match OpenOptions::new().append(true).create(true).open(file) {
        Err(e) => {
            panic!("Could not open log file: {}", e);
        }
        Ok(f) => f,
    };

    if let Err(e) = WriteLogger::init(LevelFilter::Info, Config::default(), logfile) {
        panic!("Could not initiate logging: {}", e);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    #[test]
    fn init_logs() {
        let file = format!("/tmp/clinte_{}.log", *user::NAME);
        let blank = " ".bytes().collect::<Vec<u8>>();
        fs::write(&file, &blank).unwrap();
        init().unwrap();

        log::info!("TEST LOG MESSAGE");
        let logfile = fs::read_to_string(&file).unwrap();
        assert!(logfile.contains("TEST LOG MESSAGE"));
    }
}
