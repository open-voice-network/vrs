#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

mod route_handlers;
mod utils;

use crate::route_handlers::{
    records
};

fn main() {
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
