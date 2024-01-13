use crate::classes::user_model::model::UserCreate;
use crate::errors::errors::MyError;
use crate::services::user::UserService;

use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use rocket::State;

use serde::ser::Error;
use sqlx::{Pool, Postgres};
#[post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    credentials: Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, Status> {
    let username = credentials["username"].as_str().unwrap();
    let password = credentials["password"].as_str().unwrap();

    // Add the return
    if username == "admin" && password == "password" {
        Ok(Json(serde_json::json!({
            "msg": "Login successful"
        })))
    } else {
        Err(Status::Unauthorized)
    }
}

#[post("/register", format = "json", data = "<credentials>")]
pub async fn register(
    pool: &State<Pool<Postgres>>,
    credentials: Json<serde_json::Value>,
) -> Result<Status, MyError> {
    let username = credentials["username"].as_str().unwrap();
    let password = credentials["password"].as_str().unwrap();

    if username.is_empty() || password.is_empty() {
        return Err(MyError::BadRequest {
            msg: "Username or password is empty".to_string(),
            source: std::fmt::Error,
        });
    }

    let new_user = UserCreate {
        username: Some(username.to_string()),
        password: Some(password.to_string()),
    };

    let reg = UserService::register(pool, new_user).await;

    match reg {
        Ok(_) => Ok(Status::Created),
        Err(err) => {
            // output to log or stdout
            println!("{}", err);
            // return to user
            Err(MyError::SerdeJson {
                source: serde_json::Error::custom(err.to_string())
            })
        }
    }
}
