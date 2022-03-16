use postgres::GenericClient;
use rocket::http::Status;
use sqlx::{Pool, Postgres};

use crate::models::{Todo, TodoCreate};
pub struct TodoService {}
impl TodoService {
    pub async fn fetch_all(pool: &Pool<Postgres>) -> Result<Vec<Todo>, sqlx::Error> {
        let todos: Vec<Todo> = sqlx::query_as!(Todo, r"select * from todos")
            .fetch_all(&*pool)
            .await?;
        Ok(todos)
    }
    pub async fn fetch_id(id: i32, pool: &Pool<Postgres>) -> Result<Todo, sqlx::Error> {
        let todos = sqlx::query_as!(Todo, " select * from todos where id = $1 ", id)
            .fetch_one(&*pool)
            .await?;
        Ok(todos)
    }
    pub async fn create_todo(todo: TodoCreate, pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        sqlx::query("insert into todos (task) values ($1) ")
            .bind(todo.task)
            .execute(&*pool)
            .await?;
            Ok(())
    }
    // TODO update
    pub async fn update_todo(todo: TodoCreate, pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        sqlx::query("update set into todos (task) values ($1) ")
            .bind(todo.task)
            .execute(&*pool)
            .await?;
            Ok(())
    }

    // TODO delete
}
