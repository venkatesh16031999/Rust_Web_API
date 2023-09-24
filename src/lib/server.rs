use rocket::{Rocket, Build};

use crate::controllers::get_routes;

pub fn get_rocket_config() -> Rocket<Build> {
    rocket::build()
    .mount("/", get_routes())
}