use std::env;

use dotenvy::{dotenv, from_filename};



pub struct Config {
    pub db_url: String,
}

impl Default for Config {
    fn default() -> Self {
        dotenv().ok();

        if from_filename(".env").is_err() {
            from_filename("store/.env").ok();
        }

        let db_url = env::var("DATABASE_URL").unwrap_or_else( |_| panic!("Missing DATABASE_URL environment variable"));
        Self {
            db_url,
        }
    }
}