use crate::utils::*;
use rocket_dyn_templates::Template;

#[get("/get_user")]
pub fn get_user() -> Template {
    let context = serde_json::json!({"logged-in": false});
    Template::render("index", &context)
}