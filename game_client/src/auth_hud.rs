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

use crate::dto::UserStatusDto;

const HOST: &str = "http://localhost:8080";

#[derive(GodotClass)]
#[class(base=Control)]
pub struct AuthHud {
    client: Client,
    base: Base<Control>,
}

#[godot_api]
impl AuthHud {
    #[func]
    fn on_ready(&mut self) {
        let mut auth_layer = self.base_mut().get_node_as::<CanvasLayer>("AuthLayer");
        let mut player_list_layer = self
            .base_mut()
            .get_node_as::<CanvasLayer>("PlayersListLayer");

        auth_layer.set_visible(true);
        player_list_layer.set_visible(false);
        godot_print_rich!("Set layers: OK");
    }

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
    fn on_players_request(&mut self) {
        godot_print_rich!("Pla pressed");
        let player_list_layer = self
            .base_mut()
            .get_node_as::<CanvasLayer>("PlayersListLayer");
        let mut player_item_list = player_list_layer.get_node_as::<ItemList>("PlayerList");
        let res = self.client.get(format!("{}/{}", HOST, "user/all")).send();
        let player_list: Vec<UserStatusDto> = match res {
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
                    "{} => {}",
                    player.login,
                    if player.is_online {
                        "online"
                    } else {
                        "offline"
                    }
                )
                .trim(),
            );
        }
    }

    fn handle_ok_response(&mut self, response: &mut Response, label: &mut Label) {
        let mut body: String = String::new();
        let _ = response.read_to_string(&mut body);
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
            .get_node_as::<Button>("PlayersRequestButton")
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
impl IControl for AuthHud {
    fn init(base: Base<Self::Base>) -> Self {
        godot_print_rich!("Init Hud: Begin");
        let client = reqwest::blocking::Client::new();
        let hud = AuthHud { client, base };
        godot_print_rich!("Init Hud: OK");
        hud
    }

    fn ready(&mut self) {
        let auth_layer = self.base_mut().get_node_as::<CanvasLayer>("AuthLayer");
        let mut login_input = auth_layer.get_node_as::<LineEdit>("LoginInput");
        login_input.grab_focus();
    }
}
