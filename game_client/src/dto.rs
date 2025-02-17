use std::collections::HashMap;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSessionRequestDto {
    pub user_id: i64,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStateDto {
    pub game_state_id: i64,
    pub user_session_id: String,
    pub active_user_id: i64,
    pub game_state: GameState,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub score: GameScore,
    pub board: Vec<Vec<Option<bool>>>,
    pub colors: HashMap<i64, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameScore {
    pub black: i32,
    pub white: i32,
}
