pub mod appstate {

    pub struct AppState {
        pub secret: String,
    }

    impl Default for AppState {
       fn default() -> Self {
           AppState {
             secret:  std::env::var("SECRET_KEY").unwrap(),
           }
       }
    }
}