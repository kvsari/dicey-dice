//! Contain the hexagonal grid using cube coordinates.

use std::collections::HashMap;

use coordinate::{Cube, IntoCube, DIRECTION, PointDirection, Axial};
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

    pub fn from_cube(cube: Cube, data: T) -> Self {
        Hexagon {
            grid_loc: cube,
            data: data,
        }
    }

    pub fn from_axial(axial: Axial, data: T) -> Self {
        Hexagon {
            grid_loc: axial.cube().unwrap(),
            data: data,
        }
    }

    pub fn grid_loc(&self) -> Cube {
        self.grid_loc
    }

    pub fn data(&self) -> &T {
        &self.data
    }
}

/*
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Orientation {
    Top,
    Left,
}
 */

fn generate_new_row(length: i32) -> Vec<Cube> {
    let first = Cube::new(0, 0, 0).unwrap();
    let mut row: Vec<Cube> = Vec::new();
    
    for x in 0..length {
        if x == 0 {
            row.push(first);
        } else {
            let previous = row[x as usize - 1];
            row.push(previous + DIRECTION[PointDirection::Right as usize]);
        }
    }

    row
}

fn row_down_right_from_row(row: &Vec<Cube>) -> Vec<Cube> {
    let mut new_row: Vec<Cube> = Vec::new();
    
    for hex in row.iter() {
        new_row.push(*hex + DIRECTION[PointDirection::DownRight as usize]);
    }

    new_row
}

fn row_down_left_from_row(row: &Vec<Cube>) -> Vec<Cube> {
    let mut new_row: Vec<Cube> = Vec::new();
    
    for hex in row.iter() {
        new_row.push(*hex + DIRECTION[PointDirection::DownLeft as usize]);
    }

    new_row
}

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

        if rows > 0 && columns > 0 {
            let first_row = generate_new_row(columns);
            hexes.extend(first_row.iter().map(|h| (*h, Hexagon::new(h, d).unwrap())));
            for row in 1..rows {
                if row % 2 == 0 {
                } else {
                }
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
    fn generate_row() {
        let row = generate_new_row(4);
        assert!(row.len() == 4);
        assert!(row[0] == (0, 0).into());
        assert!(row[1] == (1, 0).into());
        assert!(row[2] == (2, 0).into());
        assert!(row[3] == (3, 0).into());
    }

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

        let origin = Cube::new(1, -1, 0).unwrap();
        let hexagon = r_grid.fetch(origin).unwrap();        
        assert!(origin == hexagon.grid_loc());
    }

    #[test]
    fn rect_grid_1x4() {
        let r_grid = Rectangular::generate(4, 1, 4);

        let origin = Cube::new(0, 0, 0).unwrap();
        let hexagon = r_grid.fetch(origin).unwrap();
        assert!(origin == hexagon.grid_loc());

        let last = Cube::new(3, -3, 0).unwrap();
        let hexagon = r_grid.fetch(last).unwrap();
        assert!(last == hexagon.grid_loc());
    }

    #[test]
    fn rect_grid_2x2() {
        let r_grid = Rectangular::generate(2, 2, 4);

        let origin = Cube::new(0, 0, 0).unwrap();
        let hexagon = r_grid.fetch(origin).unwrap();
        assert!(origin == hexagon.grid_loc());

        let origin = Cube::new(0, -1, 1).unwrap();
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
