use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct CreateUserDto {
    pub(crate) login: String,
    pub(crate) password: String
}

#[derive(Serialize, Deserialize)]
pub(crate) struct LoginDto {
    pub(crate) login: String,
    pub(crate) password: String
}

#[derive(Serialize, Deserialize)]
pub(crate) struct LoginResponse {
    pub(crate) token: String
}

impl LoginResponse {
    pub(crate) fn new(token: String) -> Self {
        LoginResponse { token }
    }
}