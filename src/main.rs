#[macro_use] extern crate rocket;

mod controllers;
mod models;
use controllers::{signin, signup};
use crate::models::User;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let mut USERS: Vec<User> = Vec::new();
    
    rocket::build()
        .mount("/", routes![signin, signup])
        .launch()
        .await
}
