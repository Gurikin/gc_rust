use godot::{
    classes::{CanvasLayer, Control, Label, Timer},
    prelude::*,
};

use crate::{
    board::Board,
    dto::{
        GameScore, GameState, GameStateDto, StepDto, UserSessionDto, UserSessionRequestDto,
        UserSessionStepDto, UserStepRequestDto, UserTokenDto,
    },
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
    #[func]
    fn on_user_step(&mut self, row: i32, col: i32) {
        let step = StepDto { row, col };
        let session = UserSessionStepDto {
            session_id: self.get_session_id().unwrap(),
            user_id: self.get_user_id(),
        };
        let user_step_request = UserStepRequestDto { session, step };
        let body_str = serde_json::to_string(&user_step_request).unwrap_or("{}".to_string());
        match self
            .client
            .patch(format!("{}/{}", HOST, "game/state"))
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
                self.refresh_score(&game_state.game_state.score);
                self.refresh_board(&game_state.game_state);
            }
            Err(e) => {
                godot_error!("Error: {:?}", e);
            }
        }
    }

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
        self.token.clone().map(|t| t.user_id).unwrap_or(-1) //2
    }

    fn get_session_id(&mut self) -> Option<String> {
        self.session.clone().map(|t| t.session_id) //Some(String::from("8e2db1b1-6b1a-48ae-b44b-10fe5f47ffcd"))
    }

    #[func]
    pub fn on_game_state_tick(&mut self) {
        // godot_print!("Send game state request: Begin");
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
                self.refresh_score(&game_state.game_state.score);
                self.refresh_board(&game_state.game_state);
            }
            Err(e) => {
                godot_error!("Error: {:?}", e);
            }
        }
        // godot_print!("{}:\tSend game state request: Ok", get_format_time(None));
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

    fn refresh_score(&mut self, score: &GameScore) {
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

    fn refresh_board(&mut self, game_state: &GameState) {
        let mut board = self.base().get_node_as::<Board>("Board");
        for (row_num, row) in game_state.board.iter().enumerate() {
            for (col_num, col) in row.iter().enumerate() {
                let color = match col {
                    Some(b) => {
                        if *b {
                            GString::from("black")
                        } else {
                            GString::from("white")
                        }
                    }
                    None => GString::from("none"),
                };
                board.call(
                    "on_put_stone",
                    &[
                        Variant::from(row_num as i32),
                        Variant::from(col_num as i32),
                        Variant::from(color),
                    ],
                );
            }
        }
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
