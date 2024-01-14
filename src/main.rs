use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Status};
use rocket::serde::json::Json;
use rocket::{routes, Request, Response};
use rust_api_todo::routes::categories_route::*;
use rust_api_todo::routes::todos_route::*;
use rust_api_todo::routes::user_route::*;
use rust_api_todo::util::Config;
use rust_api_todo::appstate::appstate::AppState;
use sqlx::postgres::PgPoolOptions;
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
            "POST, GET, PATCH, PUT, DELETE, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        // response.set_header(Header::new("Access-Control-Expose-Headers", "true"));
    }
}

#[get("/")]
async fn index() -> Json<&'static str> {
    Json("{\"result\" : \"Hello!\"}")
}

#[post("/json", format = "json", data = "<msg>")]
fn echo(msg: Json<serde_json::Value>) -> Json<serde_json::Value> {
    println!("-----------------------------------------");
    println!("{}", msg.to_string());
    println!("-----------------------------------------");
    msg
}

#[options("/todos")]
fn optionsx() -> Status {
    Status::Ok
}


#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let appstate = AppState::default();
    // create a pgsql pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let config = Config::default();
    rocket::build()
        .manage(appstate)
        .attach(CORS)
        .mount(
            config.base,
            routes![
                index,
                token,
                register,
                get_todos,
                get_todos_id,
                create_todo,
                update_todo,
                delete_todo,
                get_categories,
                get_category_id,
                create_category,
                update_category,
                delete_category,
                echo,
                optionsx
            ],
        )
        .manage(pool)
        .launch()
        .await
        .expect("something wrong happend at the launch!");
    Ok(())
}
