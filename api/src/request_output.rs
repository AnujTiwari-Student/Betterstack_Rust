use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct CreateWebsiteOutput {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserOutput {
    pub id: String
}

#[derive(Serialize, Deserialize)]
pub struct LoginUserOutput {
    pub jwt: String,
}