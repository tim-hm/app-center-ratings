use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("invalid user id")]
    InvalidUserId,
    #[error("failed to create user record")]
    FailedToCreateUserRecord,
    #[error("failed to delete user by instance id")]
    FailedToDeleteUserRecord,
    #[error("unknown user no auth error")]
    Unknown,
}
