use std::fs::OpenOptions;

use simplelog::*;

use crate::error;
use crate::user;

pub fn checked_init() {
    let logfile = format!("/tmp/clinte_{}.log", *user::NAME);
    error::helper(init(&logfile), "Couldn't initialize logging");
}

fn init(path: &str) -> error::Result<()> {
    let logfile = OpenOptions::new().append(true).create(true).open(path)?;
    WriteLogger::init(LevelFilter::Info, Config::default(), logfile)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::user;
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

    #[test]
    #[ignore]
    fn checked_init_logs() {
        checked_init();
        log::info!("CHECKED INIT TEST");

        let path = format!("/tmp/clinte_{}.log", *user::NAME);
        let logfile = fs::read_to_string(&path).unwrap();
        assert!(logfile.contains("CHECKED INIT TEST"));
    }
}
