use godot::prelude::*;

pub mod auth_hud;
pub mod board;

struct GoClient;

#[gdextension]
unsafe impl ExtensionLibrary for GoClient {}
