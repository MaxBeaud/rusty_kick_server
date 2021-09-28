use rocket::{Rocket, http::Status, serde::json::Json};

use crate::models::{self, UserSignUp};

#[post("/signup", data="<model>")]
pub fn index(model: Json<UserSignUp>) -> rocket::http::Status {
    
    Status::Ok
}