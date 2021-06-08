use rocket::response::status;
use rocket_contrib::json::JsonValue;

#[get("/")]
pub fn get_records() -> &'static str {
    "Hello from get!"
}

#[post("/records", format = "json")]
pub fn create_record() -> JsonValue {
    json!({"id": 1, "name":"Jane Smith"})
}