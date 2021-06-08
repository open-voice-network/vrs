#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

mod records;

#[rocket::main]
async fn main(){
    let _ = rocket::build()
        .mount("/",routes![
            records::get_records,
            records::create_record
        ])
        .launch()
        .await;
}
