use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatusDto {
    pub login: String,
    pub is_online: bool,
}
