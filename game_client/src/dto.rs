use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatusDto {
    pub login: String,
    pub is_online: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserTokenDto {
    pub login: String,
    pub token: String,
}
