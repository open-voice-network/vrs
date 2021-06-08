use rocket_contrib::json::JsonValue;

#[catch(404)]
pub fn not_found() -> JsonValue{ 
json! ("Sorry, Not found")
}