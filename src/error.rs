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
