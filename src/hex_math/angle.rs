use crate::errors::HexError;
use strum::EnumIter;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Angle {
    Forward = 0,
    Right = 1,
    RightBack = 2,
    Back = 3,
    LeftBack = 4,
    Left = 5,
}

impl Angle {
    pub fn apply_to_int(&self, num: i32) -> Result<i32, HexError> {
        match self {
            Angle::Forward => Ok(num + 1),
            Angle::Left => Ok(num + 5),
            Angle::Right => Ok(num + 10),
            Angle::LeftBack => Ok(num * 2),
            Angle::RightBack => {
                if num % 2 == 0 {
                    Ok(num / 2)
                } else {
                    Err(HexError::InvalidAngleForNumber(*self, num))
                }
            }
            _ => Err(HexError::InvalidAngle(*self)),
        }
    }
}

impl From<i32> for Angle {
    fn from(num: i32) -> Self {
        match num.rem_euclid(6) {
            0 => Angle::Forward,
            1 => Angle::Right,
            2 => Angle::RightBack,
            3 => Angle::Back,
            4 => Angle::LeftBack,
            5 => Angle::Left,
            _ => panic!("{num}"),
        }
    }
}

impl From<Angle> for char {
    fn from(angle: Angle) -> Self {
        match angle {
            Angle::Forward => 'w',
            Angle::Right => 'e',
            Angle::RightBack => 'd',
            Angle::Back => 's',
            Angle::LeftBack => 'a',
            Angle::Left => 'q',
        }
    }
}

impl TryFrom<char> for Angle {
    type Error = HexError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'w' => Ok(Angle::Forward),
            'e' => Ok(Angle::Right),
            'd' => Ok(Angle::RightBack),
            's' => Ok(Angle::Back),
            'a' => Ok(Angle::LeftBack),
            'q' => Ok(Angle::Left),
            _ => Err(HexError::InvalidChar(value)),
        }
    }
}
