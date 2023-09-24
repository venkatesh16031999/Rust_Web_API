use app::server::get_rocket_config;
use rocket::Error;

#[macro_use] extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), Error> {
    get_rocket_config().launch().await?;
    Ok(())
}