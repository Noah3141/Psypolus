use crate::utils::*;
use rocket_dyn_templates::Template;

#[get("/get_user/<id>")] //? Rocket will allow you to serve up a &str (so long as its lifetime is the length of the program), and the browser will render a basic page
pub fn get_user(id:u64) -> Template {
    let context = serde_json::json!({"logged-in": false});
    Template::render("index", &context)
}