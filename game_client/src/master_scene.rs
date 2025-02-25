use std::collections::HashMap;

use godot::{
    classes::{CanvasLayer, Control, Label, Timer},
    prelude::*,
};

use crate::{
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
    user_color: Option<bool>,
    client: Client,
    base: Base<Node2D>,
}

#[godot_api]
impl MasterScene {
    #[signal]
    fn put_stone(row: i32, col: i32, color: bool);

    #[func]
    fn on_game_start(&mut self) {
        if self.session.is_some() {
            match self
                .client
                .get(format!(
                    "{}/{}/{}",
                    HOST,
                    "session",
                    self.session.clone().unwrap().session_id
                ))
                .header("Content-Type", "application/json")
                .send()
            {
                Ok(response) => {
                    let us =
                        serde_json::from_str::<UserSessionDto>(response.text().unwrap().trim());
                    self.session.as_mut().unwrap().user2 = us.unwrap().user2;
                    godot_print!("Get User Session by Id sent");
                    if self.session.as_mut().unwrap().user2.is_some() {
                        let mut game_state_timer =
                            self.base().get_node_as::<Timer>("GameStateTimer");
                        game_state_timer.start();
                        let mut game_start_timer =
                            self.base().get_node_as::<Timer>("GameStartTimer");
                        game_start_timer.stop();
                    }
                }
                Err(e) => godot_error!("Error: {:?}", e),
            };
        }
    }

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
            Ok(_) => godot_print!("Step was sent"),
            Err(e) => godot_error!("Error: {:?}", e),
        };
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

    fn get_nick(&self) -> String {
        self.token.as_ref().unwrap().login.clone()
    }

    fn get_opponent_nick(&self) -> String {
        let session = &self.session;
        match self.token.clone().unwrap().login == session.clone().unwrap().user1.login {
            true => session
                .clone()
                .unwrap()
                .user2
                .map(|us| us.login)
                .unwrap_or("Wait...".to_string()),
            false => session.clone().unwrap().user1.login,
        }
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
                self.refresh_colors(&game_state.game_state.colors);
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

    fn refresh_colors(&mut self, colors: &HashMap<i64, bool>) {
        self.user_color = Some(*colors.get(&self.get_user_id()).unwrap());
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
        let mut white_score_label = game_info
            .get_node_as::<Label>("WhiteTitleLabel")
            .get_node_as::<Label>("WhiteScoreLabel");
        let black_score_text = self.get_black_score_label_text(score.black);
        let white_score_text = self.get_white_score_label_text(score.white);
        black_score_label.set_text(&black_score_text);
        white_score_label.set_text(&white_score_text);
    }

    fn get_black_score_label_text(&mut self, score: i32) -> String {
        match self.user_color {
            Some(color) => match color {
                true => format!("{}: {}", self.get_nick(), score),
                false => format!("{}: {}", self.get_opponent_nick(), score),
            },
            None => format!("{}: {}", "Wait...", score),
        }
    }

    fn get_white_score_label_text(&mut self, score: i32) -> String {
        match self.user_color {
            Some(color) => match color {
                false => format!("{}: {}", self.get_nick(), score),
                true => format!("{}: {}", self.get_opponent_nick(), score),
            },
            None => format!("{}: {}", "Wait...", score),
        }
    }

    fn refresh_board(&mut self, game_state: &GameState) {
        // let mut board = self.base().get_node_as::<Board>("Board");
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
                self.base_mut().emit_signal(
                    "put_stone",
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
            user_color: None,
            client,
            base,
        }
    }

    fn ready(&mut self) {
        self.session_request = Some(UserSessionRequestDto {
            user_id: self.get_user_id(),
            session_id: self.get_session_id(),
        });
        let mut game_start_timer = self.base().get_node_as::<Timer>("GameStartTimer");
        game_start_timer.start();
        // let board = self.base().get_node_as::<Board>("Board");
        // self.base_mut()
        //     .connect("put_stone", &board.callable("on_put_stone"));
    }
}
