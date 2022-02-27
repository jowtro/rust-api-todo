use sqlx::{Pool, Postgres};

use crate::models::Todo;
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
}
