use sqlx::{Pool, Postgres};
use crate::classes::user_model::model::UserCreate;
use crate::util::generate_hash;

pub struct UserService {}

impl UserService {
    pub async fn login(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        todo!("Add login")
    }

    pub async fn register(pool: &Pool<Postgres>, user: UserCreate) -> Result<(), sqlx::Error> {
        //generate hash
        let password_hash = generate_hash(&user.password.unwrap()).unwrap();

        sqlx::query("insert into todoapp.users (username, password_hash) values ($1,$2) ")
        .bind(user.username)
        .bind(password_hash)
        .execute(&*pool)
        .await?;
        Ok(())
    }
}