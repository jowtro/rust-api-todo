pub mod model {
    use serde::Deserialize;
    use serde::Serialize;
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Todo {
        pub todo_id: i32,
        pub task: Option<String>,
        pub completed: Option<bool>,
        pub category_id: Option<i32>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct TodoCreate {
        pub task: Option<String>,
    }
}
