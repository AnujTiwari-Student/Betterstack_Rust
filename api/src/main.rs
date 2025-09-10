use std::io::Error;

use poem::{get, post, handler, listener::TcpListener, web::Path, web::Json, Route, Server};

use crate::{request_input::CreateWebsiteInput, request_output::CreateWebsiteOutput};
use store::Store;
pub mod request_input;
pub mod request_output;

#[handler]
fn get_website(Path(website_id): Path<String>) -> String {
    format!("website: {}", website_id)
}


#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let url = data.url;
    println!("Creating website for url: {}", url);
    let s = Store{};
    let id =s.create_website();
    let response = CreateWebsiteOutput { 
        id
    };
    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = Route::new()
        .at("/status/:website_id", get(get_website))
        .at("/website", post(create_website));
    Server::new(TcpListener::bind("0.0.0.0:3001"))
      .run(app)
      .await
}