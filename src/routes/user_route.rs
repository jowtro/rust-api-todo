use crate::appstate::appstate::AppState;
use crate::errors::errors::MyError;
use crate::services::user::UserService;
use crate::{classes::user_model::model::UserCreate, util::verify_hash};

use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use rocket::State;

use serde::ser::Error;
use sqlx::{Pool, Postgres};

#[post("/token", format = "json", data = "<credentials>")]
pub async fn token(
    pool: &State<Pool<Postgres>>,
    appstate: &State<AppState>,
    credentials: Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, MyError> {
    let username = credentials["username"].as_str().unwrap();
    let password = credentials["password"].as_str().unwrap();

    if username.is_empty() || password.is_empty() {
        return Err(MyError::BadRequest {
            msg: "Username or password is empty".to_string(),
            source: std::fmt::Error,
        });
    }
    let user = UserService::fetch_user_by_name(username, &pool).await.unwrap();
    if user.password_hash.is_none() {
        return Err(MyError::Unauthorized {
            msg: "Incorrect username or password".to_string(),
            source: std::fmt::Error,
        });
    }
    let is_valid = verify_hash(password, user.password_hash.as_ref().unwrap()).unwrap();

    // Add the return
    if is_valid {
        // generate token
        let token = UserService::generate_token(&pool, user.user_id, &appstate.secret).await.unwrap();
        Ok(Json(serde_json::json!({
            "x-access-token": token
        })))
    } else {
        Err(MyError::Unauthorized {
            msg: "Incorrect username or password".to_string(),
            source: std::fmt::Error,
        })
    }
}

#[post("/register", format = "json", data = "<credentials>")]
pub async fn register(pool: &State<Pool<Postgres>>, credentials: Json<serde_json::Value>) -> Result<Status, MyError> {
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
                source: serde_json::Error::custom(err.to_string()),
            })
        }
    }
}
