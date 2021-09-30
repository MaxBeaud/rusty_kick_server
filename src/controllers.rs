use rocket::{State, http::{Cookie, CookieJar, Status}, serde::json::Json};
use crate::{models::{User, UserSignIn, UserSignUp}, services::UserService};


#[post("/signin", data="<model>")]
pub fn signin(model: Option<Json<UserSignIn>>, cookies: &CookieJar<'_>, service: &State<UserService>) -> rocket::http::Status {
    
    match &model {
        Some(e) => {
            for user in service.get_users() {
                if user.username == e.username && user.password == e.password {
                    cookies.add_private(Cookie::new("user", e.username.to_string()));
                    return Status::Accepted;
                }
            }
            Status::Unauthorized
        },
        None => {
            let cookie = cookies.get_private("user").unwrap_or_else(|| Cookie::new("none", "none"));

            if cookie.value() != "none" && service.contains_user(cookie.value()) {
                Status::Accepted
            }
            else {
                Status::Unauthorized
            }
        }
    }   
}

#[post("/signup", data="<model>")]
pub fn signup(model: Json<UserSignUp>, service: &State<UserService>) -> rocket::http::Status {
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
