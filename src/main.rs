#[macro_use] extern crate rocket;

use dotenv::dotenv;

pub mod guards;
pub mod handlers;
pub mod routes;

use crate::routes::{alexa_wrapper_v1_endpoint, bussiness, skill_endpoint};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    let _rocket = rocket::build()
        .mount(
            "/",
            routes![skill_endpoint, alexa_wrapper_v1_endpoint, bussiness],
        )
        .launch()
        .await?;
    Ok(())
}
