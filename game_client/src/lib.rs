use godot::prelude::*;

pub mod auth_hud;
pub mod board;
pub mod dto;

struct GoClient;

#[gdextension]
unsafe impl ExtensionLibrary for GoClient {}
