//hashes passwords

use argon2::Config;

pub fn hash_password(password: &str) -> String {
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), b"randomsalt", &config).unwrap()
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    argon2::verify_encoded(hash, password.as_bytes()).unwrap()
}