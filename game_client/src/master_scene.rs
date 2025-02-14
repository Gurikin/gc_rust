use godot::{
    classes::{CanvasLayer, Control},
    prelude::*,
};

use crate::dto::UserSessionDto;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct MasterScene {
    session: Option<UserSessionDto>,
    base: Base<Node2D>,
}

#[godot_api]
impl MasterScene {
    pub fn init_session(&mut self, user_session: Option<UserSessionDto>) {
        godot_print!("Init session in main scene: Begin");
        self.session = user_session;
        godot_print_rich!("{:?}", self.session);
        // self.switch_scene(true);
        godot_print!("Init session in main scene: Ok");
    }

    // pub fn switch_scene(&mut self, is_game: bool) {
    //     godot_print!("Change Scene: Begin");
    //     let mut game_info_layer = self
    //         .base_mut()
    //         .get_node_as::<Control>("GameInfoControl")
    //         .get_node_as::<CanvasLayer>("GameInfo");
    //     game_info_layer.set_visible(true);
    //     godot_print!("Change Scene: Complete");
    // }
}

#[godot_api]
impl INode2D for MasterScene {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            session: None,
            base,
        }
    }

    // fn ready(&mut self) {
    //     self.switch_scene(true);
    // }
}
