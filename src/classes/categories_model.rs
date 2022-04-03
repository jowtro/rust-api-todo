pub mod model {
    use serde::Deserialize;
    use serde::Serialize;
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Category {
        pub category_id: i32,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct CategoryCreate {
        pub name: String,
    }
}
