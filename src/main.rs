use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Status};
use rocket::response::content;
use rocket::serde::json::Json;
use rocket::{Request, Response, State};
use rust_api_todo::models::{Todo, TodoCreate};
use rust_api_todo::services::TodoService;
use rust_api_todo::util::Config;
use serde_json::Value;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::{env, fmt};

#[macro_use]
extern crate rocket;

pub struct CORS;

#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/")]
async fn index() -> content::Json<&'static str> {
    content::Json("{'result' : 'Test'}")
}

#[get("/todos/<todoid>")]
async fn get_todos_id(todoid: i32, pool: &State<Pool<Postgres>>) -> Result<Json<Todo>, Status> {
    let todo = TodoService::fetch_id(todoid, &pool).await;
    match todo {
        Ok(todo) => Ok(Json(todo)),
        _ => Err(Status::NotFound),
    }
}

#[get("/todos")]
async fn get_todos(pool: &State<Pool<Postgres>>) -> Result<Json<Vec<Todo>>, Status> {
    let todos = TodoService::fetch_all(&pool).await;
    match todos {
        Ok(todos) => Ok(Json(todos)),
        _ => Err(Status::NotFound),
    }
}

#[post("/todos", data = "<todo_payload>")]
async fn create_todo(
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

#[put("/todos", data = "<todo_payload>")]
async fn update_todo(
    todo_payload: Json<Todo>,
    pool: &State<Pool<Postgres>>,
) -> Result<Status, Status> {
    let result = TodoService::update_todo(todo_payload.0, &pool).await;
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

#[delete("/todos/<todoid>")]
async fn delete_todo(
    todoid: i32,
    pool: &State<Pool<Postgres>>,
) -> Result<Status, Status> {
    let result = TodoService::delete_todo(todoid, &pool).await;
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

#[post("/json", format = "json", data = "<msg>")]
fn echo(msg: Json<Value>) -> Json<Value> {
    println!("-----------------------------------------");
    println!("{}", msg.to_string());
    println!("-----------------------------------------");
    msg
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    // create a pgsql pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let config = Config::default();
    rocket::build()
        .attach(CORS)
        .mount(
            config.base,
            routes![
                index,
                get_todos,
                get_todos_id,
                create_todo,
                update_todo,
                delete_todo,
                echo
            ],
        )
        .manage(pool)
        .launch()
        .await
        .expect("something wrong happend at the launch!");
    Ok(())
}
