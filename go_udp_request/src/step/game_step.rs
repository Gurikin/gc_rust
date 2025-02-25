use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct PlayerColor(bool);

impl From<bool> for PlayerColor {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<PlayerColor> for bool {
    fn from(value: PlayerColor) -> Self {
        value.0
    }
}

impl Display for PlayerColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = if self.0 { "black" } else { "white" };
        write!(f, "{}", color)
    }
}

#[derive(Debug, PartialEq)]
pub struct X(i32);

#[derive(Debug, PartialEq)]
pub struct Y(i32);

#[derive(Debug, PartialEq)]
pub struct PlayerStep {
    pub x: X,
    pub y: Y,
    pub color: PlayerColor,
}

impl From<f32> for X {
    fn from(value: f32) -> Self {
        Self(value.round() as i32)
    }
}

impl From<X> for f32 {
    fn from(value: X) -> Self {
        value.0 as f32
    }
}

impl From<f32> for Y {
    fn from(value: f32) -> Self {
        Self(value.round() as i32)
    }
}

impl From<Y> for f32 {
    fn from(value: Y) -> Self {
        value.0 as f32
    }
}
