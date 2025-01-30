use godot::prelude::*;

pub mod board;
pub mod auth_hud;

struct GoClient;

#[gdextension]
unsafe impl ExtensionLibrary for GoClient {}
