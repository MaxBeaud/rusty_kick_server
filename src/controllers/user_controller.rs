use rocket::{State, http::{Cookie, CookieJar, Status}, serde::json::Json};
use crate::models::user::{User, UserSignIn, UserSignUp};
use crate::service::KickService;

#[post("/signin", data="<model>")]
pub async fn signin(model: Option<Json<UserSignIn>>, jar: &CookieJar<'_>, service: &State<KickService>) -> (Status, Json<String>) {
    
    match model {
        Some(e) => {
            for user in service.get_users() {
                if user.username == e.username && user.password == e.password {
                    jar.add_private(Cookie::new("USER", e.username.to_string()));
                    return (Status::Accepted, Json(user.username));
                }
            }
            (Status::Unauthorized, Json(String::from("WOOPELAYE")))
        },
        None => {
            let cookie = jar.get_private_pending("USER");
            match cookie {
                Some(cookie) => {
                    if service.contains_user(cookie.value()) {
                        return (Status::Accepted, Json(cookie.value().to_string()));
                    }
                    (Status::Unauthorized, Json(String::from("WOOPELAYE")))
                }
                None => {
                    (Status::Unauthorized, Json(String::from("WOOPELAYE")))
                }
            }
        }
    }   
}

#[post("/signup", data="<model>")]
pub async fn signup(model: Option<Json<UserSignUp>>, jar: &CookieJar<'_>, service: &State<KickService>) -> (Status, Json<String>) {       

    match model {      
        Some(e) => {
            let user_to_add = User::new(&e.username, &e.password);
            if service.contains_user(&user_to_add.username) {
                return (Status::Conflict, Json(String::from("Compte existe déjà")));
            }
            else if e.password == e.password_confirm {        
                service.add(user_to_add);
                jar.add_private(Cookie::new("USER", e.username.clone()));
                return (Status::Created, Json(e.username.clone()));
            }
            (Status::Unauthorized, Json(String::from("Mauvais MP")))
        },
        None => {
            (Status::Unauthorized, Json(String::from("Pas de model valide")))
        }
    }
}

#[post("/signout")]
pub async fn signout(jar: &CookieJar<'_>) -> (Status, Json<String>) {       
    jar.remove_private(Cookie::named("USER"));
    (Status::Ok, Json(String::from("")))
}

#[get("/userlist")]
pub async fn userlist(service: &State<KickService>) -> Json<Vec<User>> {       
    Json(service.get_users())
}