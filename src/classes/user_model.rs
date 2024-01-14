pub mod model {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct User {
        pub user_id: i32,
        pub username: Option<String>,
        pub password_hash: Option<String>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct UserCreate {
        pub username: Option<String>,
        pub password: Option<String>,
    }
}
