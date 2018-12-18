//! Contain the hexagonal grid using cube coordinates.

use std::convert;
use std::collections::HashMap;

use coordinate::{Cube, IntoCube, Axial};
use errors::*;

pub struct Hexagon<T> {
    grid_loc: Cube,
    data: T,    
}

impl<T> Hexagon<T> {
    pub fn new<C: IntoCube>(
        location: C, data: T
    ) -> Result<Self, FailsZeroConstraint> {
        Ok(Hexagon {
            grid_loc: location.cube()?,
            data: data,
        })
    }

    pub fn grid_loc(&self) -> Cube {
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
    columns: i32,
    rows: i32,
    //base_orientation: Orientation,
    hexes: HashMap<Cube, Hexagon<T>>,
}

impl<T: Copy> Rectangular<T> {
    pub fn generate(columns: u32, rows: u32, d: T) -> Rectangular<T> {
        let mut hexes: HashMap<Cube, Hexagon<T>> = HashMap::new();

        let rows = rows as i32;
        let columns = columns as i32;
        
        // Generate the rectangle using axial coordinates
        for row in 0..rows {
            for c in 0..columns {
                let col = (row / -2) + c;
                let coordinate = (col, (-1 * row)).cube().unwrap();
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

    pub fn fetch<C: IntoCube>(
        &self, location: C
    ) -> Result<&Hexagon<T>, BadCoordinate> {
        let coordinate = location.cube()?;
        self.hexes
            .get(&coordinate)
            .ok_or(NoHexAtCoordinate::from(coordinate).into())
    }
}

#[cfg(test)]
mod test {
    use super::*;    

    #[test]
    fn rect_grid_1x1() {
        let r_grid = Rectangular::generate(1, 1, 4);

        let origin = Cube::new(0, 0, 0).unwrap();
        let hexagon = r_grid.fetch(origin).unwrap();        
        assert!(origin == hexagon.grid_loc());
    }

    #[test]
    fn rect_grid_1x2() {
        let r_grid = Rectangular::generate(2, 1, 4);

        let origin = Cube::new(0, 0, 0).unwrap();
        let hexagon = r_grid.fetch(origin).unwrap();        
        assert!(origin == hexagon.grid_loc());

        let origin = Cube::new(1, 0, -1).unwrap();
        let hexagon = r_grid.fetch(origin).unwrap();        
        assert!(origin == hexagon.grid_loc());
    }

    #[test]
    fn rect_grid_1x4() {
        let r_grid = Rectangular::generate(4, 1, 4);

        let origin = Cube::new(0, 0, 0).unwrap();
        let hexagon = r_grid.fetch(origin).unwrap();
        assert!(origin == hexagon.grid_loc());

        let last = Cube::new(3, 0, -3).unwrap();
        let hexagon = r_grid.fetch(last).unwrap();
        assert!(last == hexagon.grid_loc());
    }

    #[test]
    fn rect_grid_2x2() {
        let r_grid = Rectangular::generate(2, 2, 4);

        let origin = Cube::new(0, 0, 0).unwrap();
        let hexagon = r_grid.fetch(origin).unwrap();
        assert!(origin == hexagon.grid_loc());

        let origin = Cube::new(1, -1, 0).unwrap();
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
