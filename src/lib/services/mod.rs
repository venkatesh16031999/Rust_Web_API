pub mod contact;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("User not found: {0}")]
    UserNotFound(String)
}