use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use rocket::State;
use std::fmt;

use crate::classes::user_model::model::UserCreate;
use crate::services::user::UserService;

use sqlx::{Pool, Postgres};

#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

#[post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    credentials: Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, Status> {
    let username = credentials["username"].as_str().unwrap();
    let password = credentials["password"].as_str().unwrap();

    // Add the return
    Ok(Json(serde_json::json!({

        "username": username,
        "password": password
    })))
}

#[post("/register", format = "json", data = "<credentials>")]
pub async fn register(
    pool: &State<Pool<Postgres>>,
    credentials: Json<serde_json::Value>,
) -> Result<Status, Status> {
    let username = credentials["username"].as_str().unwrap();
    let password = credentials["password"].as_str().unwrap();

    if username.is_empty() || password.is_empty() {
        return Err(Status::BadRequest);
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
            Err(Status::InternalServerError)
        }
    }
}
