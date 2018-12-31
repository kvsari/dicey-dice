//! Common errors.
use std::{fmt, error, convert};

use crate::coordinate::Cube;

/// Error when the three cube coordinates don't fulfil the 0 constraint where summing them
/// all together must equal 0. Therefore, x + y + z = 0. Error when x + y + z != 0.
#[derive(Debug, Copy, Clone)]
pub struct FailsZeroConstraint {
    x: i32,
    y: i32,
    z: i32,
}

impl FailsZeroConstraint {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        FailsZeroConstraint { x, y, z }
    }
}

impl fmt::Display for FailsZeroConstraint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Coordinates x: {}, y: {}, z: {} fail 0 constraint. Equal {}",
               self.x,
               self.y,
               self.z,
               self.x + self.y + self.z
        )
    }
}

impl error::Error for FailsZeroConstraint {
    fn description(&self) -> &str {
        "Cube coordinates fail 0 constraint of x + y + z = 0"
    }
}

#[derive(Debug, Copy, Clone)]
pub struct NoHexAtCoordinate {
    x: i32,
    y: i32,
    z: i32,
}

impl NoHexAtCoordinate {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        NoHexAtCoordinate { x, y, z }
    }
}

impl fmt::Display for NoHexAtCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No hexagon at coordinate x: {}, y: {}, z: {}", &self.x, &self.y, &self.z)
    }
}

impl error::Error for NoHexAtCoordinate {
    fn description(&self) -> &str {
        "No hexagon at supplied cube coordinates."
    }
}

impl convert::From<Cube> for NoHexAtCoordinate {
    fn from(cc: Cube) -> Self {
        NoHexAtCoordinate::new(cc.x(), cc.y(), cc.z())
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BadCoordinate {
    NotZero(FailsZeroConstraint),
    NoHex(NoHexAtCoordinate),
}

impl fmt::Display for BadCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BadCoordinate::NoHex(err) => write!(f, "{}", &err),
            BadCoordinate::NotZero(err) => write!(f, "{}", &err),
        }
    }
}

impl error::Error for BadCoordinate {
    fn description(&self) -> &str {
        "Bad coordinate."
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            BadCoordinate::NoHex(err) => Some(err),
            BadCoordinate::NotZero(err) => Some(err),
        }
    }
}

impl convert::From<NoHexAtCoordinate> for BadCoordinate {
    fn from(nhat: NoHexAtCoordinate) -> Self {
        BadCoordinate::NoHex(nhat)
    }
}

impl convert::From<FailsZeroConstraint> for BadCoordinate {
    fn from(fzc: FailsZeroConstraint) -> Self {
        BadCoordinate::NotZero(fzc)
    }
}

