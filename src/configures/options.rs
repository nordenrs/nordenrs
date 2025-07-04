use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone)]
pub struct AppState {
    // pub templates: tera::Tera,
    pub conn: DatabaseConnection,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID or email)
    pub exp: usize,  // Expiration time (as UTC timestamp)
}

#[derive(Debug, Deserialize)]
pub struct Params {
    pub page: Option<u64>,
    pub posts_per_page: Option<u64>,
}

pub fn get_jwt_secret_key() -> Vec<u8> {
    // Attempt to retrieve the JWT secret from the environment variable
    match env::var("JWT_SECRET") {
        Ok(secret) => secret.into_bytes(), // Convert the String to Vec<u8>
        Err(e) => {
            eprintln!("Error retrieving JWT secret key: {}", e);
            std::process::exit(1); // Exit the program if the secret key is not found
        }
    }
}

