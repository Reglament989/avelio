use argon2::{self, Config};
use rand::Rng;

pub fn hash(input: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(input, &salt, &config).unwrap()
}

pub fn validate(encoded: &str, password: &[u8]) -> bool {
    argon2::verify_encoded(encoded, password).unwrap()
}
