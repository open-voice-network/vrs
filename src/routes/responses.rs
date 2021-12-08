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
use rocket::response;
use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use rocket_contrib::json::JsonValue;
use rocket_contrib::json;

#[derive(Debug)]
pub struct ApiResponse {
    status: Status,
    message: JsonValue,
}
impl ApiResponse {
    pub fn ok(message: JsonValue) -> Self {
        ApiResponse {
            status: Status::Ok,
            message: message,
        }
    }
    pub fn err(message: JsonValue) -> Self {
        ApiResponse {
            status: Status::InternalServerError,
            message: message,
        }
    }
    pub fn internal_err() -> Self {
        ApiResponse {
            status: Status::InternalServerError,
            message: json!("Internal server error"),
        }
    }
}
impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.message.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}
