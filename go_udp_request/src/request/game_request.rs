use crate::step::game_step::*;

#[derive(Debug, PartialEq)]
pub struct SessionId(String);

impl Default for SessionId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

#[derive(Debug, PartialEq)]
pub struct GameRequest {
    pub player_step: PlayerStep,
    pub session: SessionId,
}

impl GameRequest {
    pub fn builder() -> RequestBuilder {
        RequestBuilder::default()
    }
}

#[derive(Default)]
pub struct RequestBuilder {
    player_step: Option<PlayerStep>,
    session: SessionId,
}

/// The player step's in a go game representation
///
/// # Examples
///
/// ```
/// use go_udp_request::request::game_request::GameRequest;
/// use go_udp_request::step::game_step::PlayerStep;
///
/// fn build(x: f32, y: f32, color: bool) -> GameRequest {
///     let player_step = PlayerStep {
///         x: x.into(),
///         y: y.into(),
///         color: color.into(),
///     };
///     GameRequest::builder().player_step(player_step).build()
/// }
///
/// let mut game_request = build(10.0, 20.0, true);
/// assert_eq!(10.0 as f32, game_request.player_step.x.into());
/// assert_eq!(20.0 as f32, game_request.player_step.y.into());
/// assert_eq!(true, game_request.player_step.color.into());
/// ```
impl RequestBuilder {
    pub fn player_step(mut self, player_step: PlayerStep) -> RequestBuilder {
        self.player_step = Some(player_step);
        self
    }

    pub fn session(mut self, session: SessionId) -> RequestBuilder {
        self.session = session;
        self
    }

    pub fn build(self) -> GameRequest {
        GameRequest {
            player_step: self.player_step.unwrap(),
            session: self.session,
        }
    }
}
