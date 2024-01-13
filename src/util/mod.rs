use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
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
