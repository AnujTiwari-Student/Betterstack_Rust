use crate::{request_input::{CreateWebsiteInput}, request_output::{CreateWebsiteOutput, GetWebsiteOutput}};

use std::{sync::{Arc, Mutex}};

use poem::{handler, web::{Data, Json, Path}};

use store::store::Store;


#[handler]
pub fn get_website(Path(website_id): Path<String>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<GetWebsiteOutput> {
    let mut locaked_s = s.lock().unwrap();
    let website = locaked_s.get_website(website_id).unwrap();
    let response = GetWebsiteOutput { 
        url: website.url
    };
    Json(response)
}


#[handler]
pub fn create_website(Json(data): Json<CreateWebsiteInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<CreateWebsiteOutput> {
    println!("Creating website for url: {}", data.url);
    let mut locaked_s = s.lock().unwrap();
    let website = locaked_s.create_website(String::from("d364af73-92c5-466a-85b7-709f8d6d915e"), data.url).unwrap();
    let response = CreateWebsiteOutput { 
        id: website.id
    };
    Json(response)
}