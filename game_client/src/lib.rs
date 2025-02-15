use godot::prelude::*;

pub mod board;
pub mod dto;
pub mod master_scene;
pub mod start_game_hud;
pub mod game_data;

struct GoClient;

#[gdextension]
unsafe impl ExtensionLibrary for GoClient {}
