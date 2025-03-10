use std::{collections::HashMap, io::Read};

use godot::{
    classes::{Button, CanvasLayer, Control, IControl, ItemList, Label, LineEdit},
    global::Error,
    prelude::*,
};
use reqwest::{
    blocking::{Client, Response},
    StatusCode,
};

use crate::{
    dto::{UserSessionDto, UserSessionRequestDto, UserTokenDto},
    master_scene::MasterScene,
};

const HOST: &str = "http://localhost:8080";

#[derive(GodotClass)]
#[class(base=Control)]
pub struct StartGameHud {
    client: Client,
    user_token: Option<UserTokenDto>,
    base: Base<Control>,
}

#[godot_api]
impl StartGameHud {
    #[func]
    fn on_signup(&mut self) {
        godot_print_rich!("Signup pressed");
        let auth_layer = self.base_mut().get_node_as::<CanvasLayer>("AuthLayer");
        let mut sign_error_label = auth_layer.get_node_as::<Label>("SignError");

        let login = self.get_login();
        let pass = self.get_pass();
        if login.is_err() || pass.is_err() {
            return;
        }
        match self.send_auth_request(login.unwrap(), pass.unwrap(), "signup") {
            Ok(mut response) => match response.status() {
                StatusCode::OK => self.handle_ok_response(&mut response, &mut sign_error_label),
                default => sign_error_label.set_text(
                    format!(
                        "Failed to signup. Try another credentials. Status: {}",
                        default
                    )
                    .trim(),
                ),
            },
            Err(e) => {
                sign_error_label.set_text("Failed to signup. Try another credentials.");
                godot_error!("Error on signup: {}", e)
            }
        };
    }

    #[func]
    fn on_signin(&mut self) {
        godot_print_rich!("Signin pressed");
        let auth_layer = self.base_mut().get_node_as::<CanvasLayer>("AuthLayer");
        let mut sign_error_label = auth_layer.get_node_as::<Label>("SignError");

        let login = self.get_login();
        let pass = self.get_pass();
        if login.is_err() || pass.is_err() {
            return;
        }
        match self.send_auth_request(login.unwrap(), pass.unwrap(), "signin") {
            Ok(mut response) => match response.status() {
                StatusCode::OK => self.handle_ok_response(&mut response, &mut sign_error_label),
                default => sign_error_label.set_text(
                    format!(
                        "Failed to signin. Try another credentials. Status: {}",
                        default
                    )
                    .trim(),
                ),
            },
            Err(e) => {
                sign_error_label.set_text("Failed to sign in. Try another credentials.");
                godot_error!("Error on signin: {}", e)
            }
        };
    }

    #[func]
    fn on_vacant_sessions_request(&mut self) {
        godot_print_rich!("Get Vacant sessions pressed");
        let player_list_layer = self
            .base_mut()
            .get_node_as::<CanvasLayer>("PlayersListLayer");
        let mut player_item_list = player_list_layer.get_node_as::<ItemList>("PlayerList");
        let token = self.user_token.clone().unwrap();
        let body = serde_json::to_string(&token).unwrap_or("{}".to_string());
        let res = self
            .client
            .get(format!("{}/{}", HOST, "session"))
            .body(body)
            .header("Content-Type", "application/json")
            .send();
        let player_list: Vec<UserSessionDto> = match res {
            Ok(mut response) => {
                let mut body: String = String::new();
                let _ = response.read_to_string(&mut body);
                godot_print!("{}", &body);
                serde_json::from_str(&body)
                    .map_err(|e| godot_print!("{}", e))
                    .unwrap_or(vec![])
            }
            Err(e) => {
                godot_error!("{}", e);
                vec![]
            }
        };
        if player_list.is_empty() {
            return;
        }
        for player in player_list.iter() {
            godot_print!("{:?}", &player);
            player_item_list.add_item(
                format!(
                    "{}=>{}=>{}",
                    player.user1.login,
                    if player.user1.is_online {
                        "online"
                    } else {
                        "offline"
                    },
                    player.session_id
                )
                .trim(),
            );
        }
    }

    #[func]
    fn on_create_sessions_request(&mut self) {
        godot_print_rich!("Get Vacant sessions pressed");
        let player_list_layer = self
            .base_mut()
            .get_node_as::<CanvasLayer>("PlayersListLayer");
        let user_id = self.user_token.clone().unwrap().user_id;
        let body = UserSessionRequestDto {
            user_id,
            session_id: None,
        };
        let body = serde_json::to_string(&body).unwrap_or("{}".to_string());
        let res = self
            .client
            .post(format!("{}/{}", HOST, "session"))
            .body(body)
            .header("Content-Type", "application/json")
            .send();
        match res {
            Ok(user_session_response) =>
            /*go to game scene*/
            {
                let master_scene: Gd<PackedScene> = load("res://content/scenes/Master.tscn");
                let mut master_scene = master_scene.instantiate_as::<MasterScene>();
                master_scene.bind_mut().init_game_data(
                    serde_json::from_str::<Option<UserSessionDto>>(
                        user_session_response.text().unwrap().as_str(),
                    )
                    .unwrap(),
                    self.user_token.clone(),
                );
                self.base()
                    .get_tree()
                    .and_then(|t| t.get_root())
                    .unwrap()
                    .add_child(&master_scene);
                player_list_layer.clone().set_visible(false);
            }
            Err(e) => godot_error!("{}", e),
        };
    }

    #[func]
    fn on_double_click_join_session(
        &mut self,
        index: i32,
        _at_position: Vector2,
        _mouse_button_index: i32,
    ) {
        godot_print_rich!("Join to session clicked");
        let player_list_layer = self
            .base_mut()
            .get_node_as::<CanvasLayer>("PlayersListLayer");
        let player_item_list = player_list_layer.get_node_as::<ItemList>("PlayerList");
        let item = player_item_list.get_item_text(index);
        let session_id = item.split("=>").get(2).map(|s| s.to_string());
        if session_id.is_none() {
            return;
        }
        let user_id = self.user_token.clone().unwrap().user_id;
        let body = UserSessionRequestDto {
            user_id,
            session_id,
        };
        let body = serde_json::to_string(&body).unwrap_or("{}".to_string());
        let res = self
            .client
            .patch(format!("{}/{}", HOST, "session"))
            .body(body)
            .header("Content-Type", "application/json")
            .send();
        match res {
            Ok(user_session_response) =>
            /*go to game scene*/
            {
                let master_scene: Gd<PackedScene> = load("res://content/scenes/Master.tscn");
                let mut master_scene = master_scene.instantiate_as::<MasterScene>();
                master_scene.bind_mut().init_game_data(
                    serde_json::from_str::<Option<UserSessionDto>>(
                        user_session_response.text().unwrap().as_str(),
                    )
                    .unwrap(),
                    self.user_token.clone(),
                );
                self.base()
                    .get_tree()
                    .and_then(|t| t.get_root())
                    .unwrap()
                    .add_child(&master_scene);
                player_list_layer.clone().set_visible(false);
            }
            Err(e) => godot_error!("{}", e),
        };
    }

    // #[func]
    // pub fn free_scene(&mut self) {
    //     self.base().get_node_as::<Self>("/root/Hud").free();
    // }

    fn handle_ok_response(&mut self, response: &mut Response, label: &mut Label) {
        let mut body: String = String::new();
        let _ = response.read_to_string(&mut body);
        let user_token = serde_json::from_str::<UserTokenDto>(body.trim());
        self.user_token = user_token.ok();
        godot_print_rich!("Body: {}", body);
        godot_print_rich!("Sign response: {:?}", &response);
        godot_print_rich!("Sign status: {}", response.status());
        label.set_text("");
        self.goto_players_list();
    }

    fn goto_players_list(&mut self) {
        let mut auth_layer = self.base_mut().get_node_as::<CanvasLayer>("AuthLayer");
        let mut player_list_layer = self
            .base_mut()
            .get_node_as::<CanvasLayer>("PlayersListLayer");

        auth_layer.set_visible(false);
        player_list_layer.set_visible(true);
        player_list_layer
            .get_node_as::<Button>("SessionsRequestButton")
            .grab_focus();
        godot_print_rich!("Switch layers: OK");
    }

    fn get_login(&mut self) -> Result<String, Error> {
        let auth_layer = self.base_mut().get_node_as::<CanvasLayer>("AuthLayer");
        let mut login_input = auth_layer.get_node_as::<LineEdit>("LoginInput");
        let login = login_input.get_text().to_string();
        if login.len() < 3 {
            login_input.set_placeholder("Login length must be >= 3");
            return Err(Error::ERR_INVALID_DATA);
        }
        Ok(login)
    }

    fn get_pass(&mut self) -> Result<String, Error> {
        let auth_layer = self.base_mut().get_node_as::<CanvasLayer>("AuthLayer");
        let mut pass_input = auth_layer.get_node_as::<LineEdit>("PassInput");
        let pass = pass_input.get_text().to_string();
        if pass.len() < 4 {
            pass_input.set_placeholder("Password length must be >= 4");
            return Err(Error::ERR_INVALID_DATA);
        }
        Ok(pass)
    }

    fn send_auth_request(
        &mut self,
        login: String,
        pass: String,
        uri: &str,
    ) -> Result<Response, reqwest::Error> {
        let mut body = HashMap::new();
        body.insert("login", login);
        body.insert("pass", pass);
        let sign_res = self
            .client
            .post(format!("{}/{}", HOST, uri))
            .json(&body)
            .send();
        match sign_res {
            Ok(response) => Ok(response),
            Err(e) => Err(e),
        }
    }
}

#[godot_api]
impl IControl for StartGameHud {
    fn init(base: Base<Self::Base>) -> Self {
        godot_print_rich!("Init Hud: Begin");
        let client = reqwest::blocking::Client::new();
        let hud = StartGameHud {
            client,
            user_token: None,
            base,
        };
        godot_print_rich!("Init Hud: OK");
        hud
    }

    fn ready(&mut self) {
        let mut auth_layer = self.base_mut().get_node_as::<CanvasLayer>("AuthLayer");
        let mut player_list_layer = self
            .base_mut()
            .get_node_as::<CanvasLayer>("PlayersListLayer");

        auth_layer.set_visible(true);
        player_list_layer.set_visible(false);
        godot_print_rich!("Set layers: OK");

        // let auth_layer = self.base_mut().get_node_as::<CanvasLayer>("AuthLayer");
        let mut login_input = auth_layer.get_node_as::<LineEdit>("LoginInput");
        login_input.grab_focus();
    }
}
