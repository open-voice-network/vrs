#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

mod records;
mod utils;

#[rocket::main]
async fn main(){
    let _ = rocket::build()
        .mount("/",routes![
            records::get_records,
            records::get_records_by_id,
            records::create_record,
            records::update_record,
            records::delete_record
        ])
        .register("/", catchers![utils::not_found])
        .launch()
        .await;
}
