pub mod contact;
pub mod health;
use rocket::{Route, routes, Responder};

use crate::controllers::contact::add_contact;

use crate::{services::ServiceError, repositories::DatabaseError};

pub fn get_routes() -> Vec<Route> {
    routes![add_contact]
}

pub(in crate::controllers) type Result<T> = std::result::Result<T, ApiError>;

#[derive(Responder, Debug)]
pub enum ApiError {
    #[response(status = 500, content_type = "json")]
    Service(String),
    #[response(status = 500, content_type = "json")]
    Database(String),
}

impl From<ServiceError> for ApiError {
    fn from(value: ServiceError) -> Self {
        Self::Service(format!("Service Error: {}", value))
    }
}

impl From<DatabaseError> for ApiError {
    fn from(value: DatabaseError) -> Self {
        Self::Database(format!("Database Error: {}", value))
    }
}
