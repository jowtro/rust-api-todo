use crate::classes::token_model::model::Claims;
use crate::classes::user_model::model::{User, UserCreate};
use crate::util::generate_hash;
use jsonwebtoken::{EncodingKey, Header};
use sqlx::{Pool, Postgres};
use std::time::{Duration, SystemTime, UNIX_EPOCH};


pub struct UserService {}

impl UserService {
    pub async fn login(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        todo!("Add login")
    }

    pub async fn generate_token(pool: &Pool<Postgres>, user_id: i32, secret: &String) -> Result<String, sqlx::Error> {
        let key = EncodingKey::from_secret(secret.as_ref());
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let exp: Duration = now + Duration::from_secs(120); // Set expiration time to 2 minutes from now

        let claims = Claims {
            sub: user_id.to_string(),
            iat: now.as_secs() as usize,
            exp: exp.as_secs() as i64,
        };

        let token = jsonwebtoken::encode(&Header::default(), &claims, &key).unwrap();

        sqlx::query("INSERT INTO todoapp.tokens (jwt_token, expiration_time, user_id) VALUES ($1, $2, $3)")
            .bind(&token)
            .bind(&claims.exp) // Explicitly annotate the type of `exp`
            .bind(user_id)
            .execute(&*pool)
            .await?;

        Ok(token)
    }
    pub async fn register(pool: &Pool<Postgres>, user: UserCreate) -> Result<(), sqlx::Error> {
        //generate hash
        let password_hash = generate_hash(&user.password.unwrap()).unwrap();

        sqlx::query("INSERT INTO todoapp.users (username, password_hash) VALUES ($1,$2) ")
            .bind(user.username)
            .bind(password_hash)
            .execute(&*pool)
            .await?;
        Ok(())
    }

    pub async fn fetch_user_by_name(user_name: &str, pool: &Pool<Postgres>) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(User, " SELECT * from todoapp.users where username = $1 ", user_name)
            .fetch_one(&*pool)
            .await?;
        Ok(user)
    }
}
