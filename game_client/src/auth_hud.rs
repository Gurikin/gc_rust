use std::{collections::HashMap, io::Read};

use godot::{
    classes::{CanvasLayer, Control, IControl, Label, LineEdit},
    global::Error,
    prelude::*,
};
use reqwest::blocking::{Client, Response};

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
    fn on_signin(&mut self) {
        godot_print_rich!("Signin pressed");
        let auth_layer = self.base_mut().get_node_as::<CanvasLayer>("AuthLayer");
        let mut sign_error_label = auth_layer.get_node_as::<Label>("SignError");

        let login = self.get_login();
        let pass = self.get_pass();
        if login.is_err() || pass.is_err() {
            return;
        }
        match self.send_request(login.unwrap(), pass.unwrap()) {
            Ok(mut response) => {
                let mut body: String = String::new();
                let _ = response.read_to_string(&mut body);
                godot_print_rich!("Body: {}", body);
                godot_print_rich!("Signin response: {:?}", &response);
                godot_print_rich!("Signin status: {}. Response: {}", &response.status(), &response.json::<String>().unwrap_or("No body".to_string()));
                sign_error_label.set_text("");
                // self.goto_players_list();
            },
            Err(e) => {
                sign_error_label.set_text("Failed to sign in. Try another credentials.");
                godot_error!("Error on signin: {}", e)
            },
        };
    }

    fn goto_players_list(&mut self) {
        let mut auth_layer = self.base_mut().get_node_as::<CanvasLayer>("AuthLayer");
        let mut player_list_layer = self
            .base_mut()
            .get_node_as::<CanvasLayer>("PlayersListLayer");

        auth_layer.set_visible(false);
        player_list_layer.set_visible(true);
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

    fn send_request(&mut self, login: String, pass: String) -> Result<Response, reqwest::Error> {
        let mut body = HashMap::new();
        body.insert("login", login);
        body.insert("pass", pass);
        let signin_res = self
            .client
            .post("http://localhost:8080/signin")
            .json(&body)
            .send();
        match signin_res {
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
}
