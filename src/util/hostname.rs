lazy_static! {
    pub static ref HOST_NAME: String = String::from(hostname::get().unwrap().to_str().unwrap());
}
