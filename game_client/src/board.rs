use godot::classes::{Area2D, ISprite2D, Sprite2D};
use godot::prelude::*;

use crate::stone_place::StonePlace;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Board {
    pub stone_place_vec: Vec<Vec<Gd<StonePlace>>>,
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
        // if row > self.stone_place_vec.len() {
        //     return;
        // }
        if self.stone_place_vec.get(row).is_none() {
            godot_print!("row is none");
            return;
        }
        if self.stone_place_vec.get(row).unwrap().get(col).is_none() {
            godot_print!("col is none");
            return;
        }
        // if col > self.stone_place_vec.get(row).unwrap().len() {
        //     return;
        // }
        let stone_place = self.stone_place_vec.get(row).unwrap().get(col).unwrap();
        godot_print!("row: {}; col: {}", stone_place.get_meta("Row"), stone_place.get_meta("Col"));
        godot_print!("Stone Place postition: {:?}", stone_place.get_global_position());
        let stone_path = match c {
            true => "BlackStone",
            false => "WhiteStone",
        };
        let mut stone = stone_place.get_node_as::<Sprite2D>(stone_path);
        godot_print!("Stone: {:?}", stone);
        stone.set_visible(true);
        godot_print!("Stone visibility is true: {:?}", stone);
        godot_print!("Stone postition: {:?}", stone.get_global_position());
        let mut stone = self.base().get_node_as::<Sprite2D>(stone_path);
        stone.set_visible(true);
        godot_print!("Stone visibility is true: {:?}", stone);
        godot_print!("Stone postition: {:?}", stone.get_global_position());
        
        // let stone_path = String::from("res::/content/materials/")
        //     + if c {
        //         "black_stone.svg"
        //     } else {
        //         "white_stone.svg"
        //     };
        // let stone: Gd<PackedScene> = load(stone_path.trim());
        // let mut stone = stone.instantiate_as::<Sprite2D>();
        // stone.set_position(stone_place.get_position());
        // let mut area = self.base().get_node_as::<Area2D>("Area2D");
        // area.add_child(&stone);
    }
}

#[godot_api]
impl ISprite2D for Board {
    fn init(base: Base<Self::Base>) -> Self {
        godot_print!("Make a board...");
        Self {
            stone_place_vec: vec![],
            base,
        }
    }

    fn ready(&mut self) {
        let mut area = self.base().get_node_as::<Area2D>("Area2D");
        for row in 0..19 {
            let mut col_vec: Vec<Gd<StonePlace>> = vec![];
            for col in 0..19 {
                let stone_place_scene: Gd<PackedScene> =
                    load("res://content/Framework/StonePlace.tscn");
                let mut stone_place_item = stone_place_scene.instantiate_as::<StonePlace>();
                stone_place_item.set_meta("Row", &Variant::from(row));
                stone_place_item.set_meta("Col", &Variant::from(col));
                let position = Vector2::new(row as f32 * 50.0, col as f32 * 50.0);
                stone_place_item.set_position(position);
                area.add_child(&stone_place_item);
                col_vec.push(stone_place_item);
            }
            self.stone_place_vec.push(col_vec);
        }
        // godot_print!("put vec to stone_place_vec: {:?}", self.stone_place_vec);
    }
}
