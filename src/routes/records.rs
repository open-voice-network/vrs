
use rocket::*;
use rocket_contrib::json::Json;
use rocket_contrib::json;
use rocket::response::status;

use rocket_contrib::json::JsonValue;

use r2d2_mongodb::mongodb as bson;
use r2d2_mongodb::mongodb as mongodb;

use bson::{bson, doc, Bson};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::options::{ReturnDocument, FindOneAndUpdateOptions};

// use crate::data::db::{Record, InsertableRecord, ResponseRecord, UserPassword};
use crate::data::db::{Record, InsertableRecord, ResponseRecord};
use crate::data::mongo_connection::Conn;
use crate::routes::responses::ApiResponse;

const COLLECTION: &str = "records";



#[get("/records")]
pub fn get_records() -> JsonValue {
    
    json!([{"id": 1, "name":"Jane Smith"}, {"id": 2, "name":"Jose Brinas"}])
}


#[get("/records/<id>")]
pub fn get_records_by_id(id: i32) -> JsonValue {
    json!({"id": id, "name":"Jane Smith"})
}

// #[post("/records", format = "json")]
// pub fn create_record() -> JsonValue {
//     json!({"id": 1, "name":"Jane Smith"})
// }

#[post("/records", format = "json", data = "<record>")]
pub fn create_record(connection: Conn, record: Json<InsertableRecord>) -> ApiResponse {
    let record_coll = &connection.collection(COLLECTION);
    match bson::to_bson(&Record::from_insertable((*record).clone())) {
        Ok(serialized) => {
            match serialized.as_document() {
                Some(document) => {
                    match record_coll.insert_one(document.to_owned(), None) {
                        Ok(inserted) => {
                            match inserted.inserted_id {
                                Some(id) => {
                                    match record_coll.find_one(Some(doc! { "_id":  id }), None) {
                                        Ok(find_one) => {
                                            match find_one {
                                                Some(found_record) => {
                                                    let loaded_record_doc: Result<Record, _> = bson::from_bson(Bson::Document(found_record));
                                                    match loaded_record_doc {
                                                        Ok(loaded_record) => ApiResponse::ok(json!(ResponseRecord::from_record(&loaded_record))),
                                                        Err(_) => ApiResponse::internal_err(),
                                                    }
                                                },
                                                None => ApiResponse::internal_err(),
                                            }
                                        },
                                        Err(_) => ApiResponse::internal_err(),
                                    }
                                },
                                None => match inserted.write_exception {
                                    Some(wite_error) =>{
                                        match wite_error.write_error {
                                            Some(err) =>{
                                                match err.code {
                                                    11000 => ApiResponse::err(json!("record already exists")),
                                                    _ => ApiResponse::internal_err(),
                                                }
                                            },
                                            None => ApiResponse::internal_err(),
                                        }
                                    },
                                    None => ApiResponse::internal_err(),
                                }
                            }
                        },                    
                        Err(_) => ApiResponse::internal_err(),
                    }
                },
                None => ApiResponse::internal_err(),
            }
        },
        Err(_) => ApiResponse::internal_err(),
    }
}


#[put("/records/<id>", format = "json")]
pub fn update_record(id: i32) -> JsonValue {
    json!({"id": id, "name":"Jane Smith"})
}

#[delete("/records/<_id>")]
pub fn delete_record(_id: i32) -> status::NoContent {
    status::NoContent
}
