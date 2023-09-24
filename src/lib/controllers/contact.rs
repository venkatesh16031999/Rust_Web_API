use rocket::form::Form;
use rocket::post;
use rocket::serde::json::Json;
use crate::controllers::Result;
use crate::services::contact::{create_contact, CreateContact};

#[post("/contact", data = "<contact_info>")]
pub async fn add_contact(contact_info: Form<CreateContact>) -> Result<Json<CreateContact>> {
    let contact = create_contact(contact_info).await?;

    Ok(Json(contact))
}