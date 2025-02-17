use godot::{
    classes::{CanvasLayer, Control, Label, Timer},
    prelude::*,
};

use crate::{
    dto::{GameScore, GameStateDto, UserSessionDto, UserSessionRequestDto, UserTokenDto},
    util::get_format_time,
};

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
            Ok(response) => {
                let game_state = serde_json::from_str::<GameStateDto>(
                    response.text().unwrap_or("{}".to_string()).trim(),
                )
                .unwrap();
                self.refresh_time(get_format_time(Some("%T")));
                self.refresh_score(game_state.game_state.score);
            }
            Err(e) => {
                godot_error!("Error: {:?}", e);
                return;
            }
        }
        godot_print!("{}:\tSend game state request: Ok", get_format_time(None));
    }

    #[func]
    fn refresh_time(&mut self, game_state_refresh_time: String) {
        let mut time_label = self
            .base()
            .get_node_as::<Control>("GameInfoControl")
            .get_node_as::<CanvasLayer>("GameInfo")
            .get_node_as::<Label>("TimeLabel");
        time_label.set_text(&game_state_refresh_time);
    }

    fn refresh_score(&mut self, score: GameScore) {
        let game_info = self
            .base()
            .get_node_as::<Control>("GameInfoControl")
            .get_node_as::<CanvasLayer>("GameInfo");
        let mut black_score_label = game_info
            .get_node_as::<Label>("BlackTitleLabel")
            .get_node_as::<Label>("BlackScoreLabel");
        black_score_label.set_text(&score.black.to_string());
        let mut black_score_label = game_info
            .get_node_as::<Label>("WhiteTitleLabel")
            .get_node_as::<Label>("WhiteScoreLabel");
        black_score_label.set_text(&score.white.to_string());
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
