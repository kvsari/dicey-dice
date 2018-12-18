//! Coordinate systems.

use errors::*;

pub trait IntoAxial {
    fn axial(self) -> Axial;
}

pub trait IntoCube {
    fn cube(self) -> Result<Cube, FailsZeroConstraint>;
}

impl IntoCube for (i32, i32, i32) {
    fn cube(self) -> Result<Cube, FailsZeroConstraint> {
        Cube::new(self.0, self.1, self.2)
    }
}

/// Axial coordinates can also be converted by calculating z as the negative sum of x + y.
/// Since it is always possible to convert an axial coordinate into a cube coordinate this
/// conversion will always succeed unless we are exceeding the bounds of `i64`.
impl IntoCube for (i32, i32) {
    fn cube(self) -> Result<Cube, FailsZeroConstraint> {
        let d_z = self.0 + self.1;
        Cube::new(self.0, self.1, (-1 * d_z))
    }
}

impl IntoCube for (u32, u32) {
    fn cube(self) -> Result<Cube, FailsZeroConstraint> {
        (self.0 as i32, self.1 as i32).cube()
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Axial {
    x: i32,
    y: i32,
}

impl Axial {
    pub fn new(x: i32, y: i32) -> Self {
        Axial { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

impl IntoAxial for Axial {
    fn axial(self) -> Axial {
        self
    }
}

impl IntoCube for Axial {
    fn cube(self) -> Result<Cube, FailsZeroConstraint> {
        (self.x, self.y).cube()
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    pub fn new(x: i32, y: i32, z: i32) -> Result<Self, FailsZeroConstraint> {
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
        assert!(cube.y() == 0);
        assert!(cube.z() == -1);

        let axial = Axial::new(0, 1);
        let cube = axial.cube().unwrap();
        assert!(cube.x() == 0);
        assert!(cube.y() == 1);
        assert!(cube.z() == -1);
    }
}
