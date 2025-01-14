use thiserror::Error;

use crate::hex_math::{Angle, Segment};

#[derive(Error, Debug)]
pub enum HexError {
    #[error("invalid character `{0}`")]
    InvalidChar(char),
    #[error("invalid angle `{0:?}`")]
    InvalidAngle(Angle),
    #[error("invalid angle `{0:?}` for number `{1}`")]
    InvalidAngleForNumber(Angle, i32),
    #[error("segment `{0:?}` already exists in path")]
    SegmentAlreadyExists(Segment),
}
