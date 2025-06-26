use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use crate::errors::error_message::ErrorMessage;

const MAX_PASSWORD_LENGTH: usize = 64;

fn validate_password(password: &str) -> Result<(), ErrorMessage> {
    if password.is_empty() {
        return Err(ErrorMessage::EmptyPassword);
    }
    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(ErrorMessage::ExceededMaxPasswordLength(MAX_PASSWORD_LENGTH));
    }
    Ok(())
}

pub fn hash_password(password: impl AsRef<str>) -> Result<String, ErrorMessage> {
    let password = password.as_ref();
    validate_password(password)?;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| ErrorMessage::HashingError)
        .map(|hash| hash.to_string())
}

pub fn compare(password: &str, hashed_password: &str) -> Result<bool, ErrorMessage> {
    validate_password(password)?;

    let parsed_hash =
        PasswordHash::new(hashed_password).map_err(|_| ErrorMessage::InvalidHashFormat)?;

    let compare = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(compare)
}
