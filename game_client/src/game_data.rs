// use godot::classes::{ISprite2D, InputEvent, InputEventMouse, Sprite2D};
// use godot::prelude::*;
// use reqwest::blocking::Client;

// use crate::dto::UserSessionDto;

// #[derive(GodotClass)]
// #[class(base=Node)]
// pub struct GameData {
//     session: Option<UserSessionDto>,
//     client: Client,
//     base: Base<Node>,
// }

// #[godot_api]
// impl INode for GameData {
//     fn init(base: Base<Self::Base>) -> Self {
//         Self {
//             session: None,
//             base
//         }
//     }
// }
