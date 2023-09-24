use rocket::{form::{FromForm, Form}, serde::Serialize};

use super::ServiceError;

#[derive(FromForm, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateContact {
    #[field(default = "hello")]
    pub name: String,
    #[field(default = 22)]
    pub age: i32,
    pub location: Option<String>
}

pub async fn create_contact(contact_info: Form<CreateContact>) -> Result<CreateContact, ServiceError> {
    Ok(CreateContact {
        name: contact_info.name.clone(),
        age: contact_info.age,
        location: contact_info.location.clone(),
    })
    // Err(ServiceError::UserNotFound(String::from("#JNJHDB")))
}