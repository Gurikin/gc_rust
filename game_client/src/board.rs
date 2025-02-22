use std::collections::HashMap;

use godot::classes::{Area2D, ISprite2D, Sprite2D, Texture2D};
use godot::prelude::*;

use crate::master_scene::MasterScene;
use crate::stone_place::StonePlace;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Board {
    pub stone_place_vec: HashMap<i32, HashMap<i32, Gd<StonePlace>>>,
    base: Base<Sprite2D>,
}

#[godot_api]
impl Board {
    // #[signal]
    // fn on_put_stone(row: i32, col: i32, color: bool);

    #[func]
    fn on_put_stone(&mut self, row: i32, col: i32, color: String) {
        let black = String::from("black");
        let white = String::from("white");
        let c: Option<bool>;
        if color.eq(&black) {
            c = Some(true);
        } else if color.eq(&white) {
            c = Some(false);
        } else {
            c = None;
        }
        if c.is_none() {
            return;
        }
        let c = c.unwrap();
        let row = row as usize;
        let col = col as usize;
        if !self.stone_place_vec.contains_key(&(row as i32)) {
            return;
        }
        if !self
            .stone_place_vec
            .get(&(row as i32))
            .unwrap()
            .contains_key(&(col as i32))
        {
            return;
        }
        let stone_place = self
            .stone_place_vec
            .get(&(row as i32))
            .unwrap()
            .get(&(col as i32))
            .unwrap();
        let stone_path = match c {
            true => "res://content/materials/black_stone.svg",
            false => "res://content/materials/white_stone.svg",
        };
        let mut sprite = Sprite2D::new_alloc();
        let texture: Gd<Texture2D> = load(stone_path);
        sprite.set_global_scale(Vector2::new(2.0, 2.0));
        sprite.set_texture(&texture);
        let position = Vector2::new(
            stone_place.get_position().y * 2.0,
            stone_place.get_position().x * 2.0,
        );
        sprite.set_position(position);
        let mut area = self.base().get_node_as::<Area2D>("Area2D");
        area.add_child(&sprite);
        area.remove_child(stone_place);
        self.stone_place_vec
            .get_mut(&(row as i32))
            .unwrap()
            .remove(&(col as i32));
        godot_print!("Added stone. Remove from map");
    }
}

#[godot_api]
impl ISprite2D for Board {
    fn init(base: Base<Self::Base>) -> Self {
        godot_print!("Make a board...");
        Self {
            stone_place_vec: HashMap::new(),
            base,
        }
    }

    fn ready(&mut self) {
        let mut area = self.base().get_node_as::<Area2D>("Area2D");
        let m_scn = self.base_mut().get_parent().and_then(|p| Option::from(p.cast::<MasterScene>())).expect("Master scene can't be get from board");
        for row in 0..19 {
            let mut col_vec: HashMap<i32, Gd<StonePlace>> = HashMap::new();
            for col in 0..19 {
                let stone_place_scene: Gd<PackedScene> =
                    load("res://content/framework/StonePlace.tscn");
                let mut stone_place_item = stone_place_scene.instantiate_as::<StonePlace>();
                stone_place_item.set_meta("Row", &Variant::from(row));
                stone_place_item.set_meta("Col", &Variant::from(col));
                stone_place_item.set_global_scale(Vector2::new(1.0, 1.0));
                let position = Vector2::new(row as f32 * 50.0, col as f32 * 50.0);
                stone_place_item.set_global_position(position);
                godot_print!(
                    "Added child position: {}",
                    stone_place_item.get_global_position()
                );
                stone_place_item.connect("user_step", &m_scn.callable("on_user_step"));
                area.add_child(&stone_place_item);
                col_vec.insert(col, stone_place_item);
            }
            self.stone_place_vec.insert(row, col_vec);
        }
        // godot_print!("put vec to stone_place_vec: {:?}", self.stone_place_vec);
    }
}
