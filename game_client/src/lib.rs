use godot::prelude::*;

pub mod board;
pub mod dto;
pub mod game_data;
pub mod master_scene;
pub mod start_game_hud;

struct GoClient;

#[gdextension]
unsafe impl ExtensionLibrary for GoClient {}
