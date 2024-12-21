use godot::prelude::*;

pub mod board;

struct GoClient;

#[gdextension]
unsafe impl ExtensionLibrary for GoClient {}