use sqlx::{Pool, Postgres};

use crate::classes::categories_model::model::{Category, CategoryCreate};

pub struct CategorieService {}
impl CategorieService {
    pub async fn fetch_all(pool: &Pool<Postgres>) -> Result<Vec<Category>, sqlx::Error> {
        let categories: Vec<Category> =
            sqlx::query_as!(Category, r"select * from todoapp.categories")
                .fetch_all(&*pool)
                .await?;
        Ok(categories)
    }
    pub async fn fetch_id(id: i32, pool: &Pool<Postgres>) -> Result<Category, sqlx::Error> {
        let categories = sqlx::query_as!(
            Category,
            " select * from todoapp.categories where category_id = $1 ",
            id
        )
        .fetch_one(&*pool)
        .await?;
        Ok(categories)
    }
    pub async fn create_category(
        category: CategoryCreate,
        pool: &Pool<Postgres>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("insert into todoapp.categories (name) values ($1) ")
            .bind(category.name)
            .execute(&*pool)
            .await?;
        Ok(())
    }

    pub async fn update_category(
        category_id: i32,
        category: Category,
        pool: &Pool<Postgres>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE todoapp.categories
            SET name=$1
            WHERE category_id=$2;
            "#,
            category.name,
            category_id
        )
        .execute(&*pool)
        .await?;
        Ok(())
    }

    pub async fn delete_category(
        categories_id: i32,
        pool: &Pool<Postgres>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE from todoapp.categories WHERE category_id=$1;
            "#,
            categories_id
        )
        .execute(&*pool)
        .await?;
        Ok(())
    }
}
