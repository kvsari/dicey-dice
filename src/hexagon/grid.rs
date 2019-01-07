//! Contain the hexagonal grid using cube coordinates.
use std::{fmt, iter, mem};
use std::collections::HashMap;

use super::coordinate::{Cube, IntoCube, DIRECTION, PointDirection};
use super::errors::*;

/*
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Orientation {
    Top,
    Left,
}
 */

/// References a specific hex in a hex grid. Access is guarded to prevent mutation.
#[derive(Debug, Copy, Clone)]
pub struct HexTile<'a, T> {
    coordinate: &'a Cube,
    data: &'a T,
}

impl<'a, T> HexTile<'a, T> {
    fn new(coordinate: &'a Cube, data: &'a T) -> Self {
        HexTile {
            coordinate, data
        }
    }

    pub fn coordinate(&self) -> &Cube {
        self.coordinate
    }

    pub fn data(&self) -> &T {
        self.data
    }
}

fn generate_new_row(length: i32) -> Vec<Cube> {
    let first = Cube::construct(0, 0, 0).unwrap();
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

fn row_down_right_from_row(row: &[Cube]) -> Vec<Cube> {
    let mut new_row: Vec<Cube> = Vec::new();
    
    for hex in row.iter() {
        new_row.push(*hex + DIRECTION[PointDirection::DownRight as usize]);
    }

    new_row
}

fn row_down_left_from_row(row: &[Cube]) -> Vec<Cube> {
    let mut new_row: Vec<Cube> = Vec::new();
    
    for hex in row.iter() {
        new_row.push(*hex + DIRECTION[PointDirection::DownLeft as usize]);
    }

    new_row
}

#[derive(Debug, Clone)]
pub struct Rectangular<T> {
    columns: i32,
    rows: i32,
    coordinates: Vec<Vec<Cube>>, // Store coordinates in grid to save on regenerating them.
    hexes: HashMap<Cube, T>,
}

impl<T: Copy + Clone> Rectangular<T> {
    pub fn generate(columns: u32, rows: u32, d: T) -> Rectangular<T> {
        Rectangular::generate_with(columns, rows, |_| d)
    }

    pub fn generate_with<F: FnMut(&Cube) -> T>(
        columns: u32, rows: u32, mut f: F
    ) -> Rectangular<T> {
        let mut hexes: HashMap<Cube, T > = HashMap::new();

        let rows = rows as i32;
        let columns = columns as i32;

        let mut coordinates: Vec<Vec<Cube>> = Vec::new();

        // Little ugly here with the copy-n-paste extends. TODO: Refactor me.
        if rows > 0 && columns > 0 {
            let mut last_row = generate_new_row(columns);
            coordinates.push(last_row.clone());
            hexes.extend(
                last_row
                    .iter()
                    .map(|h| {
                        let d = (f)(h);
                        (*h, d)
                    })
            );
            //println!("ROW 0: {:?}", &last_row);
            for row in 1..rows {
                last_row = if row % 2 == 0 {
                    row_down_left_from_row(&last_row)
                } else {
                    row_down_right_from_row(&last_row)
                };
                //println!("ROW {}: {:?}", &row, &last_row);
                coordinates.push(last_row.clone());
                hexes.extend(
                    last_row
                        .iter()
                        .map(|h| {
                            let d = (f)(h);
                            (*h, d)
                        })
                );
            }
        }
                
        Rectangular {
            columns, rows, coordinates, hexes,
        }
    }

    pub fn fetch<C: IntoCube>(&self, location: C) -> Result<&T, BadCoordinate> {
        let coordinate = location.cube()?;
        self.hexes
            .get(&coordinate)
            .ok_or_else(|| NoHexAtCoordinate::from(coordinate).into())
    }

    pub fn iter(&self) -> Iter<T> {
        Iter::new(self.columns as usize, self.rows as usize, &self.coordinates, &self.hexes)
    }

    /// Will clone a copy of the `Rectangular<T>` grid and iterate through all hexagons
    /// applying the sent function/closure. Function takes a reference to the coordinate
    /// that the 
    pub fn fork_with<F: FnMut(&Cube, T) -> T>(&self, mut f: F) -> Self {
        let mut clone = self.clone();

        clone
            .hexes
            .iter_mut()
            .for_each(|(cube, data)| {
                let new_data = (f)(cube, *data);
                mem::replace(data, new_data);
            });
        
        clone
    }
}

/// Simple staggered display of the hexagonal board. Use an ncurses lib for more
/// sophisticated display.
impl<T: fmt::Display> fmt::Display for Rectangular<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut counter = 0;
        let output = self.coordinates
            .iter()
            .fold(String::new(), |output, row| {
                let padding = if counter % 2 == 0 { "" } else { "  " };
                let text = row
                    .iter()
                    .fold(String::new(), |text, col| {
                        format!(
                            "{}{} ",
                            &text,
                            self.hexes
                                .get(col)
                                .expect("grid has non-existent coordinate.")
                        )
                    });                
                counter += 1;
                format!("{}{}{}\n", &output, padding, &text)
            });

        write!(f, "{}", &output)
    }
}

/*
impl<T> From<Vec<Hexagon<T>>> for Rectangular<T> {
    
}
*/

/// In order iterator for the grid.
pub struct Iter<'a, T> {
    column: usize,
    row: usize,
    columns: usize,
    rows: usize,
    coordinates: &'a [Vec<Cube>],
    hexes: &'a HashMap<Cube, T>,
}

impl<'a, T> Iter<'a, T> {
    fn new(
        columns: usize,
        rows: usize,
        coordinates: &'a [Vec<Cube>],
        hexes: &'a HashMap<Cube, T>
    ) -> Self {
        Iter {
            column: 0,
            row: 0,
            columns,
            rows,
            coordinates,
            hexes,
        }
    }
}

impl<'a, T> iter::Iterator for Iter<'a, T> {
    type Item = HexTile<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {        
        if self.column >= self.columns {
            self.row += 1;
            self.column = 0;
        }

        if self.row >= self.rows {
            return None;
        }

        let coordinate = &self.coordinates[self.row][self.column];
        self.column += 1;

        self.hexes
            .get(coordinate)
            .map(|h| HexTile::new(coordinate, h))
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

    fn increment_generator(cols: u32, rows: u32) -> Rectangular<u32> {
        let mut increment = 0;
        Rectangular::generate_with(cols, rows, |_| { increment += 1; increment })
    }

    #[test]
    fn rect_grid_1x1() {
        let r_grid = increment_generator(1, 1);

        let origin = Cube::construct(0, 0, 0).unwrap();
        assert!(*r_grid.fetch(origin).unwrap() == 1);
    }

    #[test]
    fn rect_grid_1x2() {
        let r_grid = increment_generator(2, 1);

        let origin = Cube::construct(0, 0, 0).unwrap();
        assert!(*r_grid.fetch(origin).unwrap() == 1);

        let origin = Cube::construct(1, -1, 0).unwrap();
        assert!(*r_grid.fetch(origin).unwrap() == 2);
    }

    #[test]
    fn rect_grid_1x4() {
        let r_grid = increment_generator(4, 1);

        let origin = Cube::construct(0, 0, 0).unwrap();
        assert!(*r_grid.fetch(origin).unwrap() == 1);

        let last = Cube::construct(3, -3, 0).unwrap();
        assert!(*r_grid.fetch(last).unwrap() == 4);
    }

    #[test]
    fn rect_grid_2x2() {
        let r_grid = increment_generator(2, 2);

        let origin = Cube::construct(0, 0, 0).unwrap();
        assert!(*r_grid.fetch(origin).unwrap() == 1);

        let origin = Cube::construct(1, -1, 0).unwrap();
        assert!(*r_grid.fetch(origin).unwrap() == 2);

        let origin = Cube::construct(0, -1, 1).unwrap();
        assert!(*r_grid.fetch(origin).unwrap() == 3);

        let origin = Cube::construct(1, -2, 1).unwrap();
        assert!(*r_grid.fetch(origin).unwrap() == 4);
    }

    #[test]
    fn rect_grid_3x3() {
        let r_grid = increment_generator(3, 3);

        let origin = Cube::construct(0, 0, 0).unwrap();
        assert!(*r_grid.fetch(origin).unwrap() == 1);

        let origin = Cube::construct(2, -2, 0).unwrap();
        assert!(*r_grid.fetch(origin).unwrap() == 3);

        let origin = Cube::construct(-1, -1, 2).unwrap();
        assert!(*r_grid.fetch(origin).unwrap() == 7);

        let origin = Cube::construct(1, -3, 2).unwrap();
        assert!(*r_grid.fetch(origin).unwrap() == 9);
    }

    #[test]
    fn rect_grid_4x4() {
        let r_grid = increment_generator(4, 4);

        let origin = Cube::construct(0, 0, 0).unwrap();
        assert!(*r_grid.fetch(origin).unwrap() == 1);

        let origin = Cube::construct(3, -3, 0).unwrap();
        assert!(*r_grid.fetch(origin).unwrap() == 4);

        let origin = Cube::construct(-1, -2, 3).unwrap();
        assert!(*r_grid.fetch(origin).unwrap() == 13);

        let origin = Cube::construct(2, -5, 3).unwrap();
        assert!(*r_grid.fetch(origin).unwrap() == 16);
    }

    #[test]
    fn rectangle_display_2x2() {
        let r_grid = Rectangular::generate(2, 2, 'B');
        assert_eq!("B B \n  B B \n", r_grid.to_string());
    }

    #[test]
    fn rectangle_display_3x3() {
        let r_grid = Rectangular::generate(3, 3, 'A');
        assert_eq!("A A A \n  A A A \nA A A \n", r_grid.to_string());
    }

    #[test]
    fn grid_0x0_iterator() {
        let r_grid = Rectangular::generate(0, 0, 4);
        let mut iter = r_grid.iter();
        assert!(iter.next().is_none());
    }

    #[test]
    fn grid_2x2_iterator() {
        let r_grid = increment_generator(2, 2);

        let mut iter = r_grid.iter();

        let origin = Cube::construct(0, 0, 0).unwrap();
        assert!(&origin == iter.next().unwrap().coordinate());

        let origin = Cube::construct(1, -1, 0).unwrap();
        assert!(&origin == iter.next().unwrap().coordinate());

        let origin = Cube::construct(0, -1, 1).unwrap();
        assert!(&origin == iter.next().unwrap().coordinate());

        let origin = Cube::construct(1, -2, 1).unwrap();
        assert!(&origin == iter.next().unwrap().coordinate());

        assert!(iter.next().is_none());
    }

    #[test]
    fn fork_2x2_grid() {
        let r_grid = Rectangular::generate(2, 2, 4);

        let f_grid = r_grid.fork(|h| h * 2);

        f_grid
            .iter()
            .for_each(|hex| {
                assert!(*hex.data() == 8);
            });
    }
}
