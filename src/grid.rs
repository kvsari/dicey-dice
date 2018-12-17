//! Contain the hexagonal grid using cube coordinates.

use std::convert;
use std::collections::HashMap;

use errors::*;

pub trait IntoAxialCoordinate {
    fn axial_coordinate(self) -> AxialCoordinate;
}

impl IntoAxialCoordinate for (i64, i64) {
    fn axial_coordinate(self) -> AxialCoordinate {
        AxialCoordinate::new(self.0, self.1)
    }
}

pub trait IntoCubeCoordinate {
    fn cube_coordinate(self) -> Result<CubeCoordinate, FailsZeroConstraint>;
}

impl IntoCubeCoordinate for (i64, i64, i64) {
    fn cube_coordinate(self) -> Result<CubeCoordinate, FailsZeroConstraint> {
        CubeCoordinate::new(self.0, self.1, self.2)
    }
}

/// Axial coordinates can also be converted by calculating z as the negative sum of x + y.
/// Since it is always possible to convert an axial coordinate into a cube coordinate this
/// conversion will always succeed unless we are exceeding the bounds of `i64`.
impl IntoCubeCoordinate for (i64, i64) {
    fn cube_coordinate(self) -> Result<CubeCoordinate, FailsZeroConstraint> {
        let d_z = self.0 + self.1;
        CubeCoordinate::new(self.0, self.1, (-1 * d_z))
    }
}

impl IntoCubeCoordinate for (u32, u32) {
    fn cube_coordinate(self) -> Result<CubeCoordinate, FailsZeroConstraint> {
        let d_z = self.0 + self.1;
        CubeCoordinate::new(self.0 as i64, self.1 as i64, d_z as i64)
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct AxialCoordinate {
    x: i64,
    y: i64,
}

impl AxialCoordinate {
    pub fn new(x: i64, y: i64) -> Self {
        AxialCoordinate { x, y }
    }

    pub fn x(&self) -> i64 {
        self.x
    }

    pub fn y(&self) -> i64 {
        self.y
    }
}

impl IntoAxialCoordinate for AxialCoordinate {
    fn axial_coordinate(self) -> AxialCoordinate {
        self
    }
}

impl IntoCubeCoordinate for AxialCoordinate {
    fn cube_coordinate(self) -> Result<CubeCoordinate, FailsZeroConstraint> {
        (self.x, self.y).cube_coordinate()
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

impl IntoAxialCoordinate for CubeCoordinate {
    fn axial_coordinate(self) -> AxialCoordinate {
        AxialCoordinate::new(self.x, self.y)
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
    columns: i64,
    rows: i64,
    //base_orientation: Orientation,
    hexes: HashMap<CubeCoordinate, Hexagon<T>>,
}

impl<T: Copy> Rectangular<T> {
    pub fn generate(columns: u32, rows: u32, d: T) -> Rectangular<T> {
        let mut hexes: HashMap<CubeCoordinate, Hexagon<T>> = HashMap::new();

        let rows = rows as i64;
        let columns = columns as i64;
        
        // Generate the rectangle using axial coordinates
        for row in 0..rows {
            for c in 0..columns {
                let col = (row / -2) + c;
                let coordinate = (col, (-1 * row)).cube_coordinate().unwrap();
                println!("Coordinate: {:?}", &coordinate);
                let hexagon = Hexagon::new(coordinate, d).unwrap();
                hexes.insert(coordinate, hexagon);
            }
        }
                
        Rectangular {
            columns: columns,
            rows: rows,
            hexes: hexes,
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




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn axial_into_cube() {
        let axial = AxialCoordinate::new(0, 0);
        let cube = axial.cube_coordinate().unwrap();
        assert!(cube.x() == 0);
        assert!(cube.y() == 0);
        assert!(cube.z() == 0);

        let axial = AxialCoordinate::new(1, 0);
        let cube = axial.cube_coordinate().unwrap();
        assert!(cube.x() == 1);
        assert!(cube.y() == 0);
        assert!(cube.z() == -1);

        let axial = AxialCoordinate::new(0, 1);
        let cube = axial.cube_coordinate().unwrap();
        assert!(cube.x() == 0);
        assert!(cube.y() == 1);
        assert!(cube.z() == -1);
    }

    #[test]
    fn rect_grid_1x1() {
        let r_grid = Rectangular::generate(1, 1, 4);

        let origin = CubeCoordinate::new(0, 0, 0).unwrap();
        let hexagon = r_grid.fetch(origin).unwrap();        
        assert!(origin == hexagon.grid_loc());
    }

    #[test]
    fn rect_grid_1x2() {
        let r_grid = Rectangular::generate(2, 1, 4);

        let origin = CubeCoordinate::new(0, 0, 0).unwrap();
        let hexagon = r_grid.fetch(origin).unwrap();        
        assert!(origin == hexagon.grid_loc());

        let origin = CubeCoordinate::new(1, 0, -1).unwrap();
        let hexagon = r_grid.fetch(origin).unwrap();        
        assert!(origin == hexagon.grid_loc());
    }

    #[test]
    fn rect_grid_1x4() {
        let r_grid = Rectangular::generate(4, 1, 4);

        let origin = CubeCoordinate::new(0, 0, 0).unwrap();
        let hexagon = r_grid.fetch(origin).unwrap();
        assert!(origin == hexagon.grid_loc());

        let last = CubeCoordinate::new(3, 0, -3).unwrap();
        let hexagon = r_grid.fetch(last).unwrap();
        assert!(last == hexagon.grid_loc());
    }

    #[test]
    fn rect_grid_2x2() {
        let r_grid = Rectangular::generate(2, 2, 4);

        let origin = CubeCoordinate::new(0, 0, 0).unwrap();
        let hexagon = r_grid.fetch(origin).unwrap();
        assert!(origin == hexagon.grid_loc());

        let origin = CubeCoordinate::new(1, -1, 0).unwrap();
        let hexagon = r_grid.fetch(origin).unwrap();
        assert!(origin == hexagon.grid_loc());
    }

    /*
    #[test]
    fn rect_grid_4x4() {
        let r_grid = Rectangular::generate(4, 4, 4);

        let origin = CubeCoordinate::new(0, 0, 0).unwrap();
        let hexagon = r_grid.fetch(origin).unwrap();
        assert!(origin == hexagon.grid_loc());

        
    }
    */
}
