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

#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_attributes)]

#[macro_use] use rocket::*;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::helmet::SpaceHelmet;

//TODO: As we mature we can turn this on . Turning this off so the warning will not be triggered
// use log::LevelFilter;
// use log4rs::append::console::ConsoleAppender;
// use log4rs::config::{Appender, Config, Root};
// use log4rs::encode::pattern::PatternEncoder;
// use std::{env, io, process};

pub mod routes;
pub mod data;

pub fn rocket_builder() -> rocket::Rocket {

    //launch the logger
    // log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    rocket::ignite().attach(SpaceHelmet::default())
    .mount("/", routes![routes::health::health_fn])
    .mount("/api", routes![
        routes::records::get_records,
        routes::records::get_records_by_id,
        routes::records::get_records_by_invocation_name,
        routes::records::create_record,
        routes::records::delete_record,
        routes::records::update_record,
        routes::users::get_users,
        routes::users::create_user,
        routes::users::get_users_by_id,
        routes::users::update_user,
        routes::users::delete_user,
        routes::users::patch_user_rt,
        routes::users::get_users_by_email,
        routes::auth::login_user,
    ])
    .mount("/files", StaticFiles::from("static/"))
    .manage(data::mongo_connection::init_pool())
}
