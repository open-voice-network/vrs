use rocket_contrib::json::JsonValue;

// #[get("/health")]
// pub fn health() -> JsonValue {
//     json!("OK")
// }

#[catch(404)]
pub fn not_found() -> JsonValue {
    json!(["Sorry, Not found"])
}
