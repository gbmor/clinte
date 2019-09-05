pub fn helper<T, V>(res: Result<T, V>) -> T
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
        let ok: Result<&str, &str> = Ok("okay");
        let rhs = helper(ok);
        assert_eq!("okay", rhs);
    }

    #[test]
    #[should_panic]
    fn should_panic() {
        let err: Result<&str, &str> = Err("oops");
        helper(err);
    }
}
