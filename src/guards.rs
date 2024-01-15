pub mod guards {
    use jsonwebtoken::{DecodingKey, Validation};
    use rocket::{
        http::Status,
        request::{FromRequest, Request, Outcome}, State,
    };
    use crate::{classes::token_model::model::Claims, appstate::appstate::AppState};

    #[rocket::async_trait]
    impl<'a> FromRequest<'a> for Claims {
        type Error = String;
    
        async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
            // get the app state
            let state = request.guard::<&State<AppState>>().await.unwrap();

            let decoding_key: DecodingKey = DecodingKey::from_secret(state.secret.as_bytes());
            let token = request.headers().get_one("x-access-token");

            let token = match token {
                Some(encoded_token) => {
                    jsonwebtoken::decode::<Claims>(encoded_token, &decoding_key, &Validation::default())
                        .ok()
                        .map(|token| token.claims)
                }
                None => None,
            };
    
            match token {
                Some(valid_token) => Outcome::Success(valid_token),
                None => Outcome::Error((
                    Status::Unauthorized,
                    "Invalid or missing API token!".to_string(),
                )),
            }
        }
    }
}

