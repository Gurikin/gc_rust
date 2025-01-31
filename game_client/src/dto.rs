use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatusDto {
    pub nick: String,
    pub is_online: bool,
}