use rocket::*;

#[get("/health")]
pub fn health_fn() -> String {
    "I am healthy!".to_string()
}