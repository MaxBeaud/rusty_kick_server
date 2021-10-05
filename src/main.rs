#[macro_use] extern crate rocket;

mod controllers;
mod models;
mod service;
use rocket::config::{Config, SecretKey};
use crate::{controllers::user_controller, service::KickService};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let rand: Vec<u8> = vec![
        1, 76, 61, 7, 41, 2, 100, 23, 22, 11,
        36, 35, 29, 79, 60, 88, 38, 12, 66, 70,
        99, 18, 73, 37, 16, 52, 47, 30, 89, 32,
        13, 54, 84, 65, 91, 72, 63, 67, 81, 82,
        44, 33, 78, 6, 56, 15, 3, 31, 40, 53, 20,
        51, 74, 48, 98, 21, 80, 93, 4, 57, 62, 8,
        26, 77, 9, 59, 46, 27, 95, 14, 97, 86, 87,
        85, 42, 92, 58, 94, 34, 75, 45, 17, 43, 49,
        96, 50, 90, 39, 24, 5, 25, 19, 64, 28, 69,
        68, 55, 83, 71, 10
    ];

    let mut config = Config::from(Config::figment());
    config.secret_key = SecretKey::generate().unwrap_or_else(|| SecretKey::from(&rand));
    
    rocket::custom(config)
        .mount("/api/id", routes![
            user_controller::signin, 
            user_controller::signup,
            user_controller::signout
        ])
        .mount("/", routes![
            user_controller::userlist
        ])
        .manage(KickService::new())
        .launch()
        .await
}
