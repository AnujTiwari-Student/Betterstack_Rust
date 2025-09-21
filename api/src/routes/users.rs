use std::{sync::{Arc, Mutex}};

use poem::{handler, http::StatusCode, web::{Data, Json}, Error};

use crate::{request_input::{CreateUserInput, LoginUserInput}, request_output::{CreateUserOutput, LoginUserOutput}};

use store::store::Store;

#[handler]
pub fn signin_user(Json(data): Json<LoginUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Result<Json<LoginUserOutput>, Error> {

    println!("=== SIGNIN HANDLER CALLED ===");
    println!("Received request for user: {}", data.username);

    println!("Signing in user: {}", data.username);
    let mut locked_s = s.lock().unwrap();
    let user_id = locked_s.signin_user(data.username, data.password);

    match user_id {
        Ok(user_id) => {
            println!("Login successful for user_id: {}", user_id);
            let response = LoginUserOutput {
                jwt: user_id
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
pub fn signup_user(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<CreateUserOutput> {
    println!("Signing up user: {}", data.username);
    let mut locked_s = s.lock().unwrap();
    let user_id = locked_s.signup_user(data.username, data.password).unwrap();
    let response = CreateUserOutput { 
        id: user_id
    };
    Json(response)
}
