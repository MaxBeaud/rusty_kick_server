use rocket::{http::Status, serde::json::Json};
use crate::models::{User, UserSignUp};

static mut USERS: Vec<User> = Vec::new();

#[post("/signin", data="<model>")]
pub fn signin(model: Json<User>) -> rocket::http::Status {
    println!("Model: {:?}",model);
    Status::Ok
}

#[post("/signup", data="<model>")]
pub fn signup(model: Json<UserSignUp>) -> rocket::http::Status {
    println!("Model: {:?}", model);
    if model.password == model.password_confirm{
        unsafe { 
            USERS.push(
                User::new(model.id, &model.username, &model.password)
            ); 
        }
    }
    Status::Ok
}