use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatusDto {
    pub login: String,
    pub is_online: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTokenDto {
    pub user_id: i64,
    pub login: String,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSessionDto {
    pub user1: UserStatusDto,
    pub user2: Option<UserStatusDto>,
    pub session_id: String,
}