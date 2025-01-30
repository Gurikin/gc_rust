use godot::{classes::{CanvasLayer, Control, IControl}, prelude::*};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct AuthHud {
    base: Base<Control>,
}

#[godot_api]
impl AuthHud {
    #[func]
    fn on_ready(&mut self) {
        let mut auth_layer = self.base_mut().get_node_as::<CanvasLayer>("AuthLayer");
        let mut player_list_layer = self.base_mut().get_node_as::<CanvasLayer>("PlayersListLayer");

        auth_layer.set_visible(true);
        player_list_layer.set_visible(false);
        godot_print_rich!("Set layers: OK");
    }
}

#[godot_api]
impl IControl for AuthHud {
    fn init(base: Base<Self::Base>) -> Self {
        godot_print_rich!("Init Hud");
        AuthHud {
            base,
        }
    }
}