pub mod routes;
pub mod utils;

use routes::{
    get_user::*,
    // list files
};

use utils::{
    file::*,
};



#[macro_use] extern crate rocket;


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let _rocket = rocket::build()
        .mount("/api/", routes![get_user])
        .launch()
        .await?;

    Ok(())
    
}