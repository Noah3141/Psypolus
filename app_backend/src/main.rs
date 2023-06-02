pub mod models;
pub mod routes;
pub mod utils;

use models::{
    *
};

use routes::{
    get_user::*,
    // list files
};

use utils::{
    file::*,
};

use rocket_dyn_templates::Template;

#[macro_use] extern crate rocket;


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let _rocket = rocket::build()
        .mount("/api/", routes![get_user])
        .attach(Template::fairing())
        .launch()
        .await?;

    Ok(())
    
}