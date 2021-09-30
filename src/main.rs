#[macro_use] extern crate rocket;

mod controllers;
mod models;
mod services;
use controllers::{signin, signup};

use crate::services::UserService;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
 
    rocket::build()
        .mount("/", routes![signin, signup])
        .manage(UserService::new())
        .launch()
        .await
}
