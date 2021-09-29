#[macro_use] extern crate rocket;

mod controllers;
mod models;
use controllers::{signin, signup};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .mount("/", routes![signin, signup])
        .launch()
        .await
}