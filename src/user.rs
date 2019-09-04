use users;

lazy_static! {
    pub static ref NAME: String = users::get_current_username()
        .unwrap()
        .into_string()
        .unwrap();
}
