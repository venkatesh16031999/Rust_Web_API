use rocket::{serde::json::Json, get};

#[get("/health")]
pub fn health() -> Json<String> {
    Json(String::from("Ok"))
}