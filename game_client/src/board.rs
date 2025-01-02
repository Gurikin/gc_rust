use godot::classes::{ISprite2D, InputEvent, InputEventMouse, Sprite2D};
use godot::prelude::*;

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
            base,
        }
    }

    fn physics_process(&mut self, _: f64) {}

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.input.is_action_pressed("put_stone") {
            if let Ok(mouse_event) = event.try_cast::<InputEventMouse>() {
                godot_print!(
                    "Left button was clicked at {},{}",
                    mouse_event.get_position().x - 484.0,
                    mouse_event.get_position().y - 64.0
                );
            }
        }
    }
}
