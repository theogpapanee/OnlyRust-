//hashes passwords

use argon2::Config;

pub fn hash_password(password: &str) -> String {
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), b"randomsaltlol", &config).unwrap()
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    match argon2::verify_encoded(hash, password.as_bytes()) {
        Ok(result) => result,
        Err(err) => {
            eprintln!("Error verifying password: {:?}", err);
            false
        }
    }
}

