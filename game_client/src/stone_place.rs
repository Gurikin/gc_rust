use godot::classes::{IMarker2D, InputEvent, Marker2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Marker2D)]
pub struct StonePlace {
    input: Gd<Input>,
    base: Base<Marker2D>,
}

#[godot_api]
impl StonePlace {
    /// The signal emit when user click LBM at the free point on a board
    #[signal]
    fn user_step(row: i32, col: i32);
}

#[godot_api]
impl IMarker2D for StonePlace {
    fn init(base: Base<Self::Base>) -> Self {
        godot_print!("Make a StonePlace container...");
        Self {
            input: Input::singleton(),
            base,
        }
    }

    fn input(&mut self, _: Gd<InputEvent>) {
        if self.input.is_action_just_pressed("put_stone")
            && self
                .base()
                .get_local_mouse_position()
                .distance_to(self.base().get_position())
                <= 20.0
        {
            let row = self.base().get_meta("Row");
            let col = self.base().get_meta("Col");
            godot_print!(
                "Stone is putted to {}:{}. Position: {}",
                row,
                col,
                self.base().get_global_position()
            );
            // let mut m_scn: Gd<PackedScene> = load("res://content/scenes/Master.tscn");
            godot_print!("Emit 'user_step' signal");
            self.base_mut().emit_signal("user_step", &[col, row]);
        }
    }
}
