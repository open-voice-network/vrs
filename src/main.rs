#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

mod records;
mod utils;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            records::get_records,
            records::get_records_by_id,
            records::create_record,
            records::update_record,
            records::delete_record])
        .register(catchers![utils::not_found])
        .launch();
}
