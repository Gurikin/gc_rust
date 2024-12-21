use godot::prelude::*;
use godot::classes::{Sprite2D, ISprite2D};

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Board {
    input: Gd<Input>,
    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Board {
    fn init(base: Base<Self::Base>) -> Self {
        godot_print!("Make a board...");
        Self {
            input: Input::singleton(),
            base
        }
    }

    fn physics_process(&mut self, delta: f64) {

    }
}