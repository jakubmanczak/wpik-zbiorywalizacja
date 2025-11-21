use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use rand08::rngs::OsRng;

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let argon = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    Ok(argon.hash_password(password.as_bytes(), &salt)?.to_string())
}

pub fn verify_password(candidate: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let argon = Argon2::default();
    let hash = PasswordHash::new(hash)?;
    Ok(argon.verify_password(candidate.as_bytes(), &hash).is_ok())
}
