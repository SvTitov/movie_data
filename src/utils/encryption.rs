use anyhow::{Result, anyhow};
use argon2::{
    Argon2, PasswordHash, PasswordVerifier, password_hash::{
        PasswordHasher, SaltString, rand_core::OsRng
    }
};

pub fn encrypt_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);

    let argon = Argon2::default();
    let hash = argon.hash_password(password.as_bytes(), &salt)
        .map_err(|_| anyhow!("Cannot encrypt the password!."))?
        .to_string();

    anyhow::Ok(hash)
}

pub fn veryfy_password(candidate: &str, stored: &str) -> bool {
    let hash = match PasswordHash::new(stored) {
        Ok(hash) => hash,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(candidate.as_bytes(), &hash)
        .is_ok()
}