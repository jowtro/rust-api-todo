// src/errors.rs

pub mod errors {
    use std::fmt::Error;

    use rocket::{http::Status, response, Request};
    use serde_json::Error as SerdeJsonError;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum MyError {
        #[error("SerdeJson Error {source:?}")]
        SerdeJson {
            #[from]
            source: SerdeJsonError,
        },
        #[error("Sqlx Error {source:?}")]
        Sqlx {
            #[from]
            source: sqlx::Error,
        },
        #[error("Bad Request {source:?}")]
        BadRequest {
            msg: String,
            #[source]
            source: Error,
        },
        #[error("Unauthorized {source:?}")]
        Unauthorized {
            msg: String,
            #[source]
            source: Error,
        },
    }

    impl<'r, 'o: 'r> response::Responder<'r, 'o> for MyError {
        fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
            match self {
                MyError::SerdeJson { .. } => Status::BadRequest.respond_to(req),
                MyError::BadRequest { msg, source } => {
                    println!("Error: {} {}", msg, source);
                    Status::BadRequest.respond_to(req)
                },
                _ => Status::InternalServerError.respond_to(req),
            }
        }
    }
}
