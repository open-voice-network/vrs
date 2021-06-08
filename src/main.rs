#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

mod records;

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
        .launch()
        .await;
}
