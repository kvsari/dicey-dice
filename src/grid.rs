//! Contain the hexagonal grid using cube coordinates.

use std::{fmt, error, convert};
use std::collections::HashMap;

pub trait IntoCubeCoordinate {
    fn cube_coordinate(self) -> Result<CubeCoordinate, FailsZeroConstraint>;
}

impl IntoCubeCoordinate for (i64, i64, i64) {
    fn cube_coordinate(self) -> Result<CubeCoordinate, FailsZeroConstraint> {
        CubeCoordinate::new(self.0, self.1, self.2)
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CubeCoordinate {
    x: i64,
    y: i64,
    z: i64,
}

impl CubeCoordinate {
    pub fn new(x: i64, y: i64, z: i64) -> Result<Self, FailsZeroConstraint> {
        if x + y + z != 0 {
            return Err(FailsZeroConstraint::new(x, y, z));
        }
        Ok(CubeCoordinate { x, y, z })
    }

    pub fn x(&self) -> i64 {
        self.x
    }

    pub fn y(&self) -> i64 {
        self.y
    }

    pub fn z(&self) -> i64 {
        self.z
    }
}

impl IntoCubeCoordinate for CubeCoordinate {
    fn cube_coordinate(self) -> Result<CubeCoordinate, FailsZeroConstraint> {
        Ok(self)
    }
}

pub struct Hexagon<T> {
    grid_loc: CubeCoordinate,
    data: T,    
}

impl<T> Hexagon<T> {
    pub fn new<C: IntoCubeCoordinate>(
        location: C, data: T
    ) -> Result<Self, FailsZeroConstraint> {
        Ok(Hexagon {
            grid_loc: location.cube_coordinate()?,
            data: data,
        })
    }

    pub fn grid_loc(&self) -> CubeCoordinate {
        self.grid_loc
    }
}

/*
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Orientation {
    Top,
    Left,
}
 */

pub struct Rectangular<T> {
    base_hexes: u32,
    hex_layers: u32,
    //base_orientation: Orientation,
    hexes: HashMap<CubeCoordinate, Hexagon<T>>,
}

impl<T: Default> Rectangular<T> {
    pub fn generate(base_hexes: u32, hex_layers: u32, d: T) -> Rectangular<T> {
        Rectangular {
            base_hexes: base_hexes,
            hex_layers: hex_layers,
            hexes: HashMap::new(),
        }
    }

    pub fn fetch<C: IntoCubeCoordinate>(
        &self, location: C
    ) -> Result<&Hexagon<T>, BadCoordinate> {
        let coordinate = location.cube_coordinate()?;
        self.hexes
            .get(&coordinate)
            .ok_or(NoHexAtCoordinate::from(coordinate).into())
    }
}

/*
pub enum BadCoordinateType {
    FailsZeroConstraint
}

#[derive(Debug, Copy, Clone)]
pub struct BadCoordinate {
    error: BadCoordinateType,
    x: i64,
    y: i64,
    z: i64,
}
*/

/// Error when the three cube coordinates don't fulfil the 0 constraint where summing them
/// all together must equal 0. Therefore, x + y + z = 0. Error when x + y + z != 0.
#[derive(Debug, Copy, Clone)]
pub struct FailsZeroConstraint {
    x: i64,
    y: i64,
    z: i64,
}

impl FailsZeroConstraint {
    fn new(x: i64, y: i64, z: i64) -> Self {
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
    x: i64,
    y: i64,
    z: i64,
}

impl NoHexAtCoordinate {
    fn new(x: i64, y: i64, z: i64) -> Self {
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

impl convert::From<CubeCoordinate> for NoHexAtCoordinate {
    fn from(cc: CubeCoordinate) -> Self {
        NoHexAtCoordinate::new(cc.x, cc.y, cc.z)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rect_grid_1x1() {
        let r_grid = Rectangular::generate(1, 1, 4);

        let origin = CubeCoordinate::new(0, 0, 0).unwrap();
        let hexagon = r_grid.fetch((0, 0, 0)).unwrap();        
        assert!(origin == hexagon.grid_loc());
    }

    //fn rect_grid_2x2() -> 
}
