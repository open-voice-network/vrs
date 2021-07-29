#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use]extern crate  log;

mod route_handlers;
mod utils;

//TODO: As we mature we can turn this on . Turning this off so the warning will not be triggered
// use log::LevelFilter;
// use log4rs::append::console::ConsoleAppender;
// use log4rs::config::{Appender, Config, Root};
// use log4rs::encode::pattern::PatternEncoder;
// use std::{env, io, process};

use crate::route_handlers::{
    records
};

fn main() {

    //launch the logger
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    info!("Starting VRS");

    rocket::ignite()
        .mount("/api", routes![
            utils::health,
            records::get_records,
            records::get_records_by_id,
            records::create_record,
            records::update_record,
            records::delete_record])
        .register(catchers![utils::not_found])
        .launch();
}
