/* Copyright 2021 The Open Voice Network
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use rocket::*;
use rocket_contrib::json::Json;
use rocket_contrib::json;
use rocket::response::status;
use rocket_contrib::uuid::Uuid;

// use rocket_contrib::json::JsonValue;

use r2d2_mongodb::mongodb as bson;
use r2d2_mongodb::mongodb as mongodb;

use bson::{bson, doc, Bson};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::options::{ReturnDocument, FindOneAndUpdateOptions};

use crate::data::db::{Record, InsertableRecord, ResponseRecord};
use crate::data::mongo_connection::Conn;
use crate::routes::responses::ApiResponse;
use crate::data::security::JwtGuard;

const COLLECTION: &str = "records";


#[get("/records")]
pub fn get_records(connection: Conn, _guard : JwtGuard) -> ApiResponse {
    let record_coll = &connection.collection(COLLECTION);
    match  record_coll.count(None, None) {
        Ok(res) => ApiResponse::ok(json!([res])),
        Err(_) => ApiResponse::internal_err(),
    }   
}


#[get("/records/<id>")]
pub fn get_records_by_id(connection: Conn, id: Uuid, _guard : JwtGuard) -> ApiResponse {
    let record_coll = &connection.collection(COLLECTION);
    let id =  id.to_string();
    match record_coll.find_one(Some(doc! { "_id": id.clone() }), None) {
        Ok(find_one) => {
            match find_one {
                Some(found_record) => {
                    let found_record_doc: Result<Record, _> = bson::from_bson(Bson::Document(found_record));
                    match found_record_doc {
                        Ok(found_record) => ApiResponse::ok(json!(ResponseRecord::from_record(&found_record))),
                        Err(_) => ApiResponse::internal_err(),
                    }
                }
                None => ApiResponse::err(json!(format!("id {} not found",  id)))
            }
        },
        Err(_) => ApiResponse::internal_err(),
    }
}

#[get("/records/<invocation_name>",rank = 2)]
pub fn get_records_by_invocation_name(connection: Conn, invocation_name: String, _guard : JwtGuard) -> ApiResponse {
    let record_coll = &connection.collection(COLLECTION);
    match record_coll.find_one(Some(doc! { "invocation_name": invocation_name.clone() }), None) {
        Ok(find_one) => {
            match find_one {
                Some(found_record) => {
                    let found_record_doc: Result<Record, _> = bson::from_bson(Bson::Document(found_record));
                    match found_record_doc {
                        Ok(found_record) => ApiResponse::ok(json!(ResponseRecord::from_record(&found_record))),
                        Err(_) => ApiResponse::internal_err(),
                    }
                }
                None => ApiResponse::err(json!(format!("Invocation name {} not found",  invocation_name)))
            }
        },
        Err(_) => ApiResponse::internal_err(),
    }
}

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


#[put("/records/<id>", format = "json", data = "<record>")]
pub fn update_record(connection: Conn, record: Json<InsertableRecord>, id: Uuid, _guard : JwtGuard) -> ApiResponse {
    let record_coll = &connection.collection(COLLECTION);
    let id =  id.to_string();
    match record_coll.find_one(Some(doc! { "_id": id.clone() }), None) {
        Ok(find_one) => {
            match find_one {
                Some(found_record) => {
                    let found_record_doc: Result<Record, _> = bson::from_bson(Bson::Document(found_record));
                    match found_record_doc {
                        Ok(mut found_record) => {
                            
                                // TODO: check the security
                                
                                let insertable = found_record.update_record(&record.domain_url, &record.invocation_name, &record.organization_name,
                                    &record.organization_registrant, &record.organization_email, 
                                    &record.organization_address, &record.organization_city, &record.organization_country,
                                    &record.registered_date, &record.expiration_date,
                                    &record.location, &record.destination_url, &record.status);
                                match bson::to_bson(&insertable) {
                                    Ok(serialized) => {
                                        let document = serialized.as_document().unwrap();
                                        let mut opt = FindOneAndUpdateOptions::new();
                                        opt.return_document = Some(ReturnDocument::After);
                                        match record_coll.find_one_and_replace(
                                            doc! { "_id": id.clone() },
                                            document.to_owned(),
                                            Some(opt)
                                        ) {
                                            Ok(updated_one) => {
                                                match updated_one {
                                                    Some(updated_record) => {
                                                        let update_record_doc: Result<Record, _> = bson::from_bson(Bson::Document(updated_record));
                                                        match update_record_doc {
                                                            Ok(updated) => ApiResponse::ok(json!(ResponseRecord::from_record(&updated))),
                                                            Err(_) => ApiResponse::internal_err(),
                                                        }                                                        
                                                    },
                                                    None => ApiResponse::err(json!(format!("id {} not found",  id))),
                                                }
                                            },                    
                                            Err(_) => ApiResponse::internal_err(),
                                        }
                                    },
                                    Err(_) => ApiResponse::internal_err(),
                                }
                            
                        },
                        Err(_) => ApiResponse::internal_err(),
                    }
                },
                None => ApiResponse::err(json!(format!("id {} not found",  id))),
            }            
        },
        Err(_) => ApiResponse::internal_err(),
    }
}

#[delete("/records/<_id>")]
pub fn delete_record(_id: i32) -> status::NoContent {
    status::NoContent
}
