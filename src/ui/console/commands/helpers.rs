use std::str::FromStr;

use bevy::math::{IVec2, UVec2, Vec2};
use thiserror::Error;

#[derive(Debug, Error)]
pub(super) enum PositionParseError {
    #[error("invalid position {0}.")]
    Invalid(String),
}

pub(super) trait Position: Sized {
    type Scalar: FromStr;
    fn new(x: Self::Scalar, y: Self::Scalar) -> Self;
    fn filter_chars(c: char) -> bool;
}

impl Position for Vec2 {
    type Scalar = f32;
    fn new(x: f32, y: f32) -> Self {
        Vec2::new(x, y)
    }
    fn filter_chars(c: char) -> bool {
        c.is_numeric() || c == '.' || c == '-'
    }
}

impl Position for UVec2 {
    type Scalar = u32;
    fn new(x: u32, y: u32) -> Self {
        UVec2::new(x, y)
    }
    fn filter_chars(c: char) -> bool {
        c.is_numeric()
    }
}

impl Position for IVec2 {
    type Scalar = i32;
    fn new(x: i32, y: i32) -> Self {
        IVec2::new(x, y)
    }
    fn filter_chars(c: char) -> bool {
        c.is_numeric() || c == '-'
    }
}

pub(super) fn parse_position<P: Position>(position: &[String]) -> Result<P, PositionParseError> {
    let parse_coord = |s: &str| -> Result<P::Scalar, PositionParseError> {
        let filtered: String = s.chars().filter(|&c| P::filter_chars(c)).collect();
        filtered
            .parse::<P::Scalar>()
            .map_err(|_| PositionParseError::Invalid(s.to_string()))
    };

    let (x_str, y_str) = if let Some((x, y)) = position.first().and_then(|s| s.split_once(',')) {
        (x.to_string(), y.to_string())
    } else if position.len() >= 2 {
        (position[0].clone(), position[1].clone())
    } else {
        return Err(PositionParseError::Invalid(position.join(" ")));
    };

    let x = parse_coord(&x_str)?;
    let y = parse_coord(&y_str)?;

    Ok(P::new(x, y))
}
