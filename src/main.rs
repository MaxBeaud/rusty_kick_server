#[macro_use] extern crate rocket;

mod controllers;
mod models;

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/hello", routes![world])
        .launch()
        .await;
}