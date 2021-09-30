use rocket::{State, http::{Cookie, CookieJar, Status}, serde::json::Json};
use crate::{models::{User, UserSignUp}, services::UserService};


#[post("/signin", data="<model>")]
pub fn signin(model: Json<User>, cookies: &CookieJar<'_>, service: &State<UserService>) -> rocket::http::Status {
    
    if service.contains_user(cookies.get_private("user").unwrap().value()) {
        return Status::Accepted;
    }    
    for user in service.get_users() {
        if user.username == model.username && user.password == model.password {
            cookies.add_private(Cookie::new("user", model.username.to_string()));
            return Status::Accepted;
        }
    }
    Status::Unauthorized
}

#[post("/signup", data="<model>")]
pub fn signup(model: Json<UserSignUp>, service: State<UserService>) -> rocket::http::Status {
    println!("Model: {:?}", model);
    
    let user_to_add = User::new(service.get_users().len() as i32, &model.username, &model.password);

    if model.password == model.password_confirm && !service.contains_user(&user_to_add.username) {
        service.add(user_to_add);
    }
    else {
        return Status::Unauthorized;
    }
    Status::Created
}
