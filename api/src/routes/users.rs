use std::{sync::{Arc, Mutex}};

use poem::{handler, web::{Data, Json}};

use crate::{request_input::{CreateUserInput, LoginUserInput}, request_output::{CreateUserOutput, LoginUserOutput}};

use store::store::Store;

#[handler]
pub fn signin_user(Json(data): Json<LoginUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<LoginUserOutput> {
    println!("Signing in user: {}", data.username);
    let mut locaked_s = s.lock().unwrap();
    let _exists = locaked_s.signin_user(data.username, data.password).unwrap();
    let response = LoginUserOutput { 
        jwt: String::from("jwt")
    };
    Json(response)
}

#[handler]
pub fn signup_user(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<CreateUserOutput> {
    println!("Signing up user: {}", data.username);
    let mut locaked_s = s.lock().unwrap();
    let user_id = locaked_s.signup_user(data.username, data.password).unwrap();
    let response = CreateUserOutput { 
        id: user_id
    };
    Json(response)
}
