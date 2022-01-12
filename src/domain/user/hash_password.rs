use argon2;

pub fn execute(password: String) -> Result<String, ()> {
    let config = argon2::Config::default();

    let hashed_password = argon2::hash_encoded(password.as_bytes(), "testSalt".as_bytes(), &config);

    hashed_password.map_err(|_| ())
}
