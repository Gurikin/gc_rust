use godot::classes::{IStaticBody2D, InputEvent, StaticBody2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=StaticBody2D)]
pub struct StonePlace {
    input: Gd<Input>,
    base: Base<StaticBody2D>,
}

#[godot_api]
impl StonePlace {}

#[godot_api]
impl IStaticBody2D for StonePlace {
    fn init(base: Base<Self::Base>) -> Self {
        godot_print!("Make a StonePlace container...");
        Self {
            input: Input::singleton(),
            base,
        }
    }

    fn input(&mut self, _: Gd<InputEvent>) {
        if self.input.is_action_pressed("put_stone")
            && self
                .base()
                .get_local_mouse_position()
                .distance_to(self.base().get_position())
                <= 40.0
        {
            let row = self.base().get_meta("Row");
            let col = self.base().get_meta("Col");
            godot_print!("Stone is putted to {}:{}", row, col);
        }
    }
}
