
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use rocket::{get, delete, put, post};
use crate::classes::todo_model::model::{Todo, TodoCreate};
use crate::services::todo::TodoService;
use crate::classes::token_model::model::Claims;
use sqlx::{Pool, Postgres};

#[get("/todos/<todo_id>")]
pub async fn get_todos_id(todo_id: i32, pool: &State<Pool<Postgres>>) -> Result<Json<Todo>, Status> {
    let todo = TodoService::fetch_todo_id(todo_id, &pool).await;
    match todo {
        Ok(todo) => Ok(Json(todo)),
        _ => Err(Status::NotFound),
    }
}

#[get("/todos")]
pub async fn get_todos(pool: &State<Pool<Postgres>>, token: Claims) -> Result<Json<Vec<Todo>>, Status> {
    // check if token exists
    if token.sub.is_empty() {
        return Err(Status::Unauthorized);
    }
    if token.sub != "5" {
        return Err(Status::Unauthorized);
    }
    let todos = TodoService::fetch_all(&pool).await;
    match todos {
        Ok(todos) => Ok(Json(todos)),
        _ => Err(Status::NotFound),
    }
}

#[post("/todos", data = "<todo_payload>")]
pub async fn create_todo(
    todo_payload: Json<TodoCreate>,
    pool: &State<Pool<Postgres>>,
) -> Result<Status, Status> {
    // create Todo
    let result = TodoService::create_todo(todo_payload.0, &pool).await;
    match result {
        Ok(_) => Ok(Status::Created),
        Err(err) => {
            // output to log or stdout
            println!("{}", err);
            // return to user
            Err(Status::InternalServerError)
        }
    }
}

#[put("/todos/<todo_id>", format = "json", data = "<todo_payload>")]
pub async fn update_todo(
    todo_id : i32,
    todo_payload: Json<Todo>,
    pool: &State<Pool<Postgres>>,
) -> Result<Status, Status> {
    let result = TodoService::update_todo(todo_id, todo_payload.0, &pool).await;
    match result {
        Ok(_) => Ok(Status::Ok),
        Err(err) => {
            // output to log or stdout
            println!("{}", err);
            // return to user
            Err(Status::InternalServerError)
        }
    }
}

#[delete("/todos/<todo_id>")]
pub async fn delete_todo(todo_id: i32, pool: &State<Pool<Postgres>>) -> Result<Status, Status> {
    let result = TodoService::delete_todo(todo_id, &pool).await;
    match result {
        Ok(_) => Ok(Status::NoContent),
        Err(err) => {
            // output to log or stdout
            println!("{}", err);
            // return to user
            Err(Status::InternalServerError)
        }
    }
}