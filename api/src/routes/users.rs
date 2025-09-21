use std::{sync::{Arc, Mutex}};

use jsonwebtoken::{encode, EncodingKey, Header};
use poem::{handler, http::StatusCode, web::{Data, Json}, Error};
use serde::{Deserialize, Serialize};

use crate::{request_input::{CreateUserInput, LoginUserInput}, request_output::{CreateUserOutput, LoginUserOutput}};

use store::store::Store;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[handler]
pub fn signin_user(Json(data): Json<LoginUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Result<Json<LoginUserOutput>, Error> {

    println!("=== SIGNIN HANDLER CALLED ===");
    println!("Received request for user: {}", data.username);

    let mut locked_s = s.lock().unwrap();
    let user_id = locked_s.signin_user(data.username, data.password);

    match user_id {
        Ok(user_id) => {
            println!("Login successful for user_id: {}", user_id);

            let my_claims = Claims {
                sub: user_id,
                exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
            };

            let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref())).map_err(|_| {
                println!("Failed to encode token");
                Error::from_status(StatusCode::UNAUTHORIZED)
            })?;

            let response = LoginUserOutput {
                jwt: token
            };

            Ok(Json(response))
        }
        Err(e) => {
            println!("Login failed with error: {:?}", e);
            Err(Error::from_status(StatusCode::UNAUTHORIZED))
        }
    }
}

#[handler]
pub fn signup_user(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Result<Json<CreateUserOutput>, Error> {
    println!("Signing up user: {}", data.username);
    let mut locked_s = s.lock().unwrap();
    let user_id = locked_s.signup_user(data.username, data.password).map_err( |e| {
        println!("Signup failed with error: {:?}", e);
        Error::from_status(StatusCode::UNAUTHORIZED)
    })?;
    let response = CreateUserOutput { 
        id: user_id
    };
    Ok(Json(response))
}
