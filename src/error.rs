use crate::conf;

// This Result is used elsewhere, not in helper()
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn helper<T, V>(res: std::result::Result<T, V>, simplified_message: &str) -> T
where
    V: std::fmt::Debug,
{
    match res {
        Ok(val) => val,
        Err(err) => {
            log::error!("{}", simplified_message);
            if *conf::DEBUG {
                log::error!("--> {:?}", err);
            }
            eprintln!("{}", simplified_message);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shouldnt_panic() {
        let ok: std::result::Result<&str, &str> = Ok("okay");
        let rhs = helper(ok, "okay");
        assert_eq!("okay", rhs);
    }
}
