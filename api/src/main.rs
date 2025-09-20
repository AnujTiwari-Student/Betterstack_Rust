use std::{io::Error, sync::{Arc, Mutex}};

use poem::{get, listener::TcpListener, post, EndpointExt, Route, Server};


use crate::{routes::{users::{signin_user, signup_user}, websites::{create_website, get_website}}};

use store::store::Store;
pub mod request_input;
pub mod request_output;
pub mod routes;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let s = Arc::new(Mutex::new(Store::new().unwrap()));
    let app = Route::new()
        .at("/status/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/signup", post(signup_user))
        .at("/signin", post(signin_user))
        .data(s);
    Server::new(TcpListener::bind("0.0.0.0:3001"))
      .run(app)
      .await
}