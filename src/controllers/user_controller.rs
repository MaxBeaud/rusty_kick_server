use crate::models::user::{User, UserSignIn, UserSignUp};
use crate::service::KickService;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rocket::{
    http::{Cookie, CookieJar, Status},
    serde::json::Json,
    State,
};
use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[post("/signin", data = "<model>")]
pub fn sign_in(
    model: Option<Json<UserSignIn>>,
    jar: &CookieJar<'_>,
    service: &State<KickService>,
) -> (Status, Json<String>) {
    match model {
        Some(e) => {
            if service.get_users_val().par_iter().any(|user| user.username == e.username && user.is_valid_password(e.password.clone())) {
                jar.add_private(Cookie::new("USER", e.username.to_string()));
                return (Status::Accepted, Json(e.username.to_string()));
            }
            (
                Status::Unauthorized,
                Json(String::from("Utilisateur non valide")),
            )
        }
        None => {
            let cookie = jar.get_private_pending("USER");
            match cookie {
                Some(cookie) => {
                    if service.contains_user(cookie.value()) {
                        return (Status::Accepted, Json(cookie.value().to_string()));
                    }
                    (
                        Status::Unauthorized,
                        Json(String::from("Aucune authentification valide")),
                    )
                }
                None => (
                    Status::Unauthorized,
                    Json(String::from("Aucune authentification valide")),
                ),
            }
        }
    }
}

#[post("/signup", data = "<model>")]
pub fn sign_up(
    model: Option<Json<UserSignUp>>,
    jar: &CookieJar<'_>,
    service: &State<KickService>,
) -> (Status, Json<String>) {
    match model {
        Some(e) => {
            let user_to_add = User::new(&e.username, &e.password);
            if service.contains_user(&user_to_add.username) {
                return (Status::Conflict, Json(String::from("Compte existe d??j??")));
            } else if e.password == e.password_confirm {
                service.add_user(user_to_add);
                jar.add_private(Cookie::new("USER", e.username.clone()));
                return (Status::Created, Json(e.username.clone()));
            }
            (Status::Unauthorized, Json(String::from("Mauvais MP")))
        }
        None => (
            Status::Unauthorized,
            Json(String::from("Pas de model valide")),
        ),
    }
}

#[post("/signout")]
pub fn sign_out(jar: &CookieJar<'_>) -> (Status, Json<String>) {
    jar.remove_private(Cookie::named("USER"));
    (Status::Ok, Json(String::from("")))
}

#[get("/userlist")]
pub fn user_list(service: &State<KickService>) -> Json<Vec<User>> {
    Json(service.get_users_val())
}

#[get("/userlistpretty")]
pub fn user_list_pretty(service: &State<KickService>) -> Template {
    let mut context: HashMap<&str, Vec<User>> = HashMap::new();
    context.insert("users", service.get_users_val());
    Template::render("user_list", &context)
}

#[get("/userlistpretty/<username>")]
pub fn user_pretty(service: &State<KickService>, username: String) -> Template {
    let mut context: HashMap<&str, User> = HashMap::new();
    context.insert("user", service.get_user(&username).unwrap());
    Template::render("user", &context)
}
