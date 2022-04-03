use sqlx::{Pool, Postgres};

use crate::classes::todo_model::model::{Todo, TodoCreate};
pub struct TodoService {}
impl TodoService {
    pub async fn fetch_all(pool: &Pool<Postgres>) -> Result<Vec<Todo>, sqlx::Error> {
        let todos: Vec<Todo> = sqlx::query_as!(Todo, r"select * from todoapp.todos")
            .fetch_all(&*pool)
            .await?;
        Ok(todos)
    }
    pub async fn fetch_todo_id(todo_id: i32, pool: &Pool<Postgres>) -> Result<Todo, sqlx::Error> {
        let todos = sqlx::query_as!(
            Todo,
            " select * from todoapp.todos where todo_id = $1 ",
            todo_id
        )
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

    pub async fn update_todo(todo: Todo, pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE todoapp.todos
            SET task=$1, completed=$2
            WHERE todo_id=$3;
            "#,
            todo.task,
            todo.completed,
            todo.todo_id
        )
        .execute(&*pool)
        .await?;
        Ok(())
    }

    pub async fn delete_todo(todo_todo_id: i32, pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE from todoapp.todos WHERE todo_id=$1;
            "#,
            todo_todo_id
        )
        .execute(&*pool)
        .await?;
        Ok(())
    }
}
