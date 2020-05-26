use users;

lazy_static! {
    pub static ref NAME: String = users::get_current_username()
        .expect("Could not get username")
        .into_string()
        .expect("Could not get username");
}
