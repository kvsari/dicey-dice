//! Coordinate systems.
use std::{convert, ops, fmt};
use std::ops::Neg;

use super::errors::*;

/// The different directions as `Cube` coordinate additions. This is for both flat and
/// pointy orientations of the hexagonal grid.
pub static DIRECTION: &[Cube] = &[// Assuming pointy in comments (just as valid for flat).
    Cube { x: -1, y: 1, z: 0 },   // Left
    Cube { x: 1, y: -1, z: 0 },   // Right
    Cube { x: 0, y: 1, z: -1 },   // UpLeft
    Cube { x: 1, y: 0, z: -1 },   // UpRight
    Cube { x: -1, y: 0, z: 1 },   // DownLeft,
    Cube { x: 0, y: -1, z: 1 },   // DownRight,
];

/// A hexagon on a hexagonal grid has six directions it can go. These six directions
/// correspond to a 'pointed' oriented grid. Each movement can be added to the current
/// hexagon to get the coordinates of the new one. This is used for calculating neighbours.
/// ```ascii
///   / \      
///  /   \
/// |     |
/// |     |
///  \   /
///   \ /  
/// ```
pub enum PointDirection {
    Left = 0,
    Right = 1,
    UpLeft = 2,
    UpRight = 3,
    DownLeft = 4,
    DownRight = 5,
}

/// A hexagon on a hexagonal grid has six directions it can go. These six directions
/// correspond to a 'flat' grid. Each movement can be added to the current hexagon to get
/// the coordinates of the new one. This is used for calculating neighbours.
/// ```ascii
///   ___
///  /   \
/// /     \
/// \     /
///  \___/
/// ```
pub enum FlatDirection {
    Up = 2,
    Down = 5,
    LeftUp = 0,
    RightUp = 3,
    LeftDown = 4,
    RightDown = 1,
}

pub trait IntoAxial {
    fn axial(self) -> Axial;
}

pub trait IntoCube {
    fn cube(self) -> Result<Cube, FailsZeroConstraint>;
}

impl IntoAxial for (i32, i32) {
    fn axial(self) -> Axial {
        Axial::new(self.0, self.1)
    }
}

impl IntoAxial for (u32, u32) {
    fn axial(self) -> Axial {
        Axial::new(self.0 as i32, self.1 as i32)
    }
}

impl IntoCube for (i32, i32, i32) {
    fn cube(self) -> Result<Cube, FailsZeroConstraint> {
        Cube::construct(self.0, self.1, self.2)
    }
}

/// Axial coordinates can also be converted by calculating z as the negative sum of x + y.
/// Since it is always possible to convert an axial coordinate into a cube coordinate this
/// conversion will always succeed unless we are exceeding the bounds of `i64`.
impl IntoCube for (i32, i32) {
    fn cube(self) -> Result<Cube, FailsZeroConstraint> {
        Axial::from(self).cube()
    }
}

impl IntoCube for (u32, u32) {
    fn cube(self) -> Result<Cube, FailsZeroConstraint> {
        Axial::from((self.0 as i32, self.1 as i32)).cube()
    }
}

/// Axial coordinates simulate a (X, Y) cartesian plane but is not quite one. Therefore
/// to avoid confusion we have `c` for columns going diagonally down to the right, and rows.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Axial {
    column: i32,
    row: i32,
}

impl Axial {
    pub fn new(column: i32, row: i32) -> Self {
        Axial { column, row }
    }

    pub fn column(self) -> i32 {
        self.column
    }

    pub fn row(self) -> i32 {
        self.row
    }
}

impl IntoAxial for Axial {
    fn axial(self) -> Axial {
        self
    }
}

impl IntoCube for Axial {
    fn cube(self) -> Result<Cube, FailsZeroConstraint> {
        let y = self.column + self.row;
        Cube::construct(self.column, y.neg(), self.row)
    }
}

impl From<(i32, i32)> for Axial {
    fn from(tuple: (i32, i32)) -> Self {
        Axial::new(tuple.0, tuple.1)
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    pub fn construct(x: i32, y: i32, z: i32) -> Result<Self, FailsZeroConstraint> {
        if x + y + z != 0 {
            return Err(FailsZeroConstraint::new(x, y, z));
        }
        Ok(Cube { x, y, z })
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn z(&self) -> i32 {
        self.z
    }

    /// Return the coordinates for the six neighbours. Outputted clockwise.
    /// Whether they are valid on a grid is another question not answered here.
    pub fn neighbours(&self) -> [Cube; 6] {
        [
            *self + DIRECTION[PointDirection::UpRight as usize],
            *self + DIRECTION[PointDirection::Right as usize],
            *self + DIRECTION[PointDirection::DownRight as usize],
            *self + DIRECTION[PointDirection::DownLeft as usize],
            *self + DIRECTION[PointDirection::Left as usize],
            *self + DIRECTION[PointDirection::UpLeft as usize],
        ]
    }

    /// As above but only returns the three neighbours that are Right, DownRight and
    /// and DownLeft. This is used for certain scanning operations that need to cover
    /// the entire board yet it's possible to do it by only looking at these three
    /// adjacent tiles.
    pub fn three_neighbours(&self) -> [Cube; 3] {
        [
            *self + DIRECTION[PointDirection::Right as usize],
            *self + DIRECTION[PointDirection::DownRight as usize],
            *self + DIRECTION[PointDirection::DownLeft as usize],
        ]
    }
}

impl IntoAxial for Cube {
    fn axial(self) -> Axial {
        Axial::new(self.x, self.y)
    }
}

impl IntoCube for Cube {
    fn cube(self) -> Result<Cube, FailsZeroConstraint> {
        Ok(self)
    }
}

impl IntoCube for &Cube {
    fn cube(self) -> Result<Cube, FailsZeroConstraint> {
        Ok(*self)
    }
}

impl ops::Add for Cube {
    type Output = Cube;
    
    fn add(self, other: Cube) -> Cube {
        Cube {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Add<Axial> for Cube {
    type Output = Cube;

    fn add(self, other: Axial) -> Cube {
        self + other.cube().unwrap()
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", &self.x, &self.y, &self.z)
    }
}

impl convert::From<Axial> for Cube {
    fn from(a: Axial) -> Self {
        a.cube().unwrap()
    }
}

impl convert::From<(i32, i32)> for Cube {
    fn from(tuple: (i32, i32)) -> Self {
        tuple.cube().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn axial_into_cube() {
        let axial = Axial::new(0, 0);
        let cube = axial.cube().unwrap();
        assert!(cube.x() == 0);
        assert!(cube.y() == 0);
        assert!(cube.z() == 0);

        let axial = Axial::new(1, 0);
        let cube = axial.cube().unwrap();
        assert!(cube.x() == 1);
        assert!(cube.y() == -1);
        assert!(cube.z() == 0);

        let axial = Axial::new(0, 1);
        let cube = axial.cube().unwrap();
        assert!(cube.x() == 0);
        assert!(cube.y() == -1);
        assert!(cube.z() == 1);

        let axial = Axial::new(-1, 0);
        let cube = axial.cube().unwrap();
        assert!(cube.x() == -1);
        assert!(cube.y() == 1);
        assert!(cube.z() == 0);

        let axial = Axial::new(0, -1);
        let cube = axial.cube().unwrap();
        assert!(cube.x() == 0);
        assert!(cube.y() == 1);
        assert!(cube.z() == -1);

        let axial = Axial::new(1, -1);
        let cube = axial.cube().unwrap();
        assert!(cube.x() == 1);
        assert!(cube.y() == 0);
        assert!(cube.z() == -1);

        let axial = Axial::new(-1, -1);
        let cube = axial.cube().unwrap();
        assert!(cube.x() == -1);
        assert!(cube.y() == 2);
        assert!(cube.z() == -1);

        let axial = Axial::new(-1, 1);
        let cube = axial.cube().unwrap();
        assert!(cube.x() == -1);
        assert!(cube.y() == 0);
        assert!(cube.z() == 1);

        let axial = Axial::new(1, 1);
        let cube = axial.cube().unwrap();
        assert!(cube.x() == 1);
        assert!(cube.y() == -2);
        assert!(cube.z() == 1);
    }
}
