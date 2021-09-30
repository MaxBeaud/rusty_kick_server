use rocket::{http::Status, serde::json::Json};
use crate::models::{User, UserSignUp};

static mut USERS: Vec<User> = Vec::new();

#[post("/signin", data="<model>")]
pub fn signin(model: Json<User>) -> rocket::http::Status {
    unsafe {
        for user in USERS {
            if user.username == model.username && user.password == model.password {
                return Status::Accepted;
            }
            else {
                return Status::Forbidden;
            }
        }
    }  
    Status::Ok
}

#[post("/signup", data="<model>")]
pub fn signup(model: Json<UserSignUp>) -> rocket::http::Status {
    println!("Model: {:?}", model);
    let user_to_add = User::new(model.id, &model.username, &model.password);
    unsafe {
        if 
        model.password == model.password_confirm && 
        !USERS.contains(&user_to_add)
        {
            USERS.push(user_to_add);
        }
        else {
            return Status::Conflict;
        }
    }    
    Status::Created
}
