use std::time::SystemTime;

use chrono::{DateTime, Utc};
use godot::{classes::Timer, prelude::*};

use crate::dto::{UserSessionDto, UserSessionRequestDto, UserTokenDto};

use reqwest::blocking::Client;

const HOST: &str = "http://localhost:8080";

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct MasterScene {
    session: Option<UserSessionDto>,
    token: Option<UserTokenDto>,
    session_request: Option<UserSessionRequestDto>,
    client: Client,
    base: Base<Node2D>,
}

#[godot_api]
impl MasterScene {
    pub fn init_game_data(
        &mut self,
        user_session: Option<UserSessionDto>,
        token: Option<UserTokenDto>,
    ) {
        godot_print!("Init session in main scene: Begin");
        self.session = user_session;
        self.token = token;
        godot_print_rich!("{:?}", self.session);
        // self.switch_scene(true);
        godot_print!("Init session in main scene: Ok");
    }

    fn get_user_id(&mut self) -> i64 {
        self.token.clone().map(|t| t.user_id).unwrap_or(-1)
    }

    fn get_session_id(&mut self) -> Option<String> {
        self.session.clone().map(|t| t.session_id)
    }

    #[func]
    pub fn on_game_state_tick(&mut self) {
        godot_print!("Send game state request: Begin");
        let body_str = serde_json::to_string(&self.session_request).unwrap_or("{}".to_string());
        match self
            .client
            .post(format!("{}/{}", HOST, "game/state"))
            .body(body_str)
            .header("Content-Type", "application/json")
            .send()
        {
            Ok(response) => godot_print!("{:?}", response.text()),
            Err(e) => {
                godot_error!("Error: {:?}", e);
                return;
            }
        }
        let now = SystemTime::now();
        let datetime: DateTime<Utc> = now.into();
        println!("{}", datetime.format("%d/%m/%Y %T"));
        godot_print!("{}:\tSend game state request: Ok", datetime.format("%Y"));
    }
}

#[godot_api]
impl INode2D for MasterScene {
    fn init(base: Base<Self::Base>) -> Self {
        let client = reqwest::blocking::Client::new();
        Self {
            session: None,
            token: None,
            session_request: None,
            client,
            base,
        }
    }

    fn ready(&mut self) {
        self.session_request = Some(UserSessionRequestDto {
            user_id: self.get_user_id(),
            session_id: self.get_session_id(),
        });
        let mut game_state_timer = self.base().get_node_as::<Timer>("GameStateTimer");
        game_state_timer.start();
    }
}
