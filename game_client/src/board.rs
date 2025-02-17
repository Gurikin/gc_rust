use godot::classes::{Area2D, ISprite2D, Sprite2D};
use godot::prelude::*;

use crate::stone_place::StonePlace;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Board {
    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Board {
    fn init(base: Base<Self::Base>) -> Self {
        godot_print!("Make a board...");
        Self {
            base,
        }
    }

    fn ready(&mut self) {
        let mut area = self.base().get_node_as::<Area2D>("Area2D");
        for row in 0..19 {
            for col in 0..19 {
                let stone_place_scene: Gd<PackedScene> =
                    load("res://content/Framework/StonePlace.tscn");
                let mut stone_place_item = stone_place_scene.instantiate_as::<StonePlace>();
                stone_place_item.set_meta("Row", &Variant::from(row));
                stone_place_item.set_meta("Col", &Variant::from(col));
                let position = Vector2::new(row as f32 * 50.0, col as f32 * 50.0);
                stone_place_item.set_position(position);
                area.add_child(&stone_place_item);
            }
        }
    }
}
