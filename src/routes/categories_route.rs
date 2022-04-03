use rocket::http::{Status};
use rocket::serde::json::Json;
use rocket::{State, get, delete, put, post};
use sqlx::{Pool, Postgres};

use crate::services::categories::CategorieService;
use crate::classes::categories_model::model::{Category, CategoryCreate};

#[get("/categories/<categoryid>")]
pub async fn get_category_id(categoryid: i32, pool: &State<Pool<Postgres>>) -> Result<Json<Category>, Status> {
    let category = CategorieService::fetch_id(categoryid, &pool).await;
    match category {
        Ok(category) => Ok(Json(category)),
        _ => Err(Status::NotFound),
    }
}

#[get("/categories")]
pub async fn get_categories(pool: &State<Pool<Postgres>>) -> Result<Json<Vec<Category>>, Status> {
    let categories = CategorieService::fetch_all(&pool).await;
    match categories {
        Ok(categories) => Ok(Json(categories)),
        _ => Err(Status::NotFound),
    }
}


#[post("/categories", data = "<category_payload>")]
pub async fn create_category(
    category_payload: Json<CategoryCreate>,
    pool: &State<Pool<Postgres>>,
) -> Result<Status, Status> {
    let result = CategorieService::create_category(category_payload.0, &pool).await;
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

#[put("/categories", format = "json", data = "<category_payload>")]
pub async fn update_category(
    category_payload: Json<Category>,
    pool: &State<Pool<Postgres>>,
) -> Result<Status, Status> {
    let result = CategorieService::update_category(category_payload.0, &pool).await;
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

#[delete("/categories/<category_id>")]
pub async fn delete_category(category_id: i32, pool: &State<Pool<Postgres>>) -> Result<Status, Status> {
    let result = CategorieService::delete_category(category_id, &pool).await;
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


