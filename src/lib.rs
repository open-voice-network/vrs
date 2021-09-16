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