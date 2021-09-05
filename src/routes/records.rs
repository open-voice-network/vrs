
use rocket::*;
use rocket_contrib::json;
use rocket::response::status;

use rocket_contrib::json::JsonValue;



#[get("/records")]
pub fn get_records() -> JsonValue {
    
    json!([{"id": 1, "name":"Jane Smith"}, {"id": 2, "name":"Jose Brinas"}])
}


#[get("/records/<id>")]
pub fn get_records_by_id(id: i32) -> JsonValue {
    json!({"id": id, "name":"Jane Smith"})
}

#[post("/records", format = "json")]
pub fn create_record() -> JsonValue {
    json!({"id": 1, "name":"Jane Smith"})
}

#[put("/records/<id>", format = "json")]
pub fn update_record(id: i32) -> JsonValue {
    json!({"id": id, "name":"Jane Smith"})
}

#[delete("/records/<_id>")]
pub fn delete_record(_id: i32) -> status::NoContent {
    status::NoContent
}
