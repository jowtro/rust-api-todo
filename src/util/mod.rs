use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

pub struct Config {
    pub base: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            base: "/api/v1/".to_string(),
        }
    }
}

pub fn generate_hash(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();

    Ok(password_hash)
}

pub fn verify_hash(password: &str, password_hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(&password_hash)?;
    let is_valid = Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok();
    Ok(is_valid)
}
