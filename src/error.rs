// This Result is used elsewhere, not in helper()
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn helper<T, V>(res: std::result::Result<T, V>) -> T
where
    V: std::fmt::Debug,
{
    match res {
        Ok(val) => val,
        Err(err) => {
            log::error!("{:?}", err);
            panic!("{:?}", err);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shouldnt_panic() {
        let ok: std::result::Result<&str, &str> = Ok("okay");
        let rhs = helper(ok);
        assert_eq!("okay", rhs);
    }

    #[test]
    #[should_panic]
    fn should_panic() {
        let err: std::result::Result<&str, &str> = Err("oops");
        helper(err);
    }
}
