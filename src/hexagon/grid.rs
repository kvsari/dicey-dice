//! Contain the hexagonal grid using cube coordinates.
use std::{fmt, mem, hash};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rectangular<T> {
    columns: i32,
    rows: i32,
    hexes: Vec<(Cube, T)>,
    index: HashMap<Cube, usize>,
}

impl<T: Copy + Clone + Hash + PartialEq + Eq> Rectangular<T> {
    pub fn generate(columns: u32, rows: u32, d: T) -> Rectangular<T> {
        Rectangular::generate_with(columns, rows, |_| d)
    }

    pub fn generate_with<F: FnMut(&Cube) -> T>(
        columns: u32, rows: u32, mut f: F
    ) -> Rectangular<T> {
        let rows = rows as i32;
        let columns = columns as i32;

        if columns == 0 || rows == 0 {
            return Rectangular {
                columns,
                rows,
                hexes: Vec::new(),
                index: HashMap::new(),
            };
        }

        let mut coordinates: Vec<Cube> = Vec::new();
        let mut last_row = generate_new_row(columns);
        coordinates.extend(last_row.clone().into_iter());
        for row in 1..rows {
            last_row = if row % 2 == 0 {
                row_down_left_from_row(&last_row)
            } else {
                row_down_right_from_row(&last_row)
            };
            coordinates.extend(last_row.clone().into_iter());
        }

        let hexes: Vec<(Cube, T)> = coordinates
            .into_iter()
            .map(|c| (c, (f)(&c)))
            .collect();

        let index = hexes
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut map, (i, (c, _))| {
                map.insert(*c, i);
                map
            });
                
        Rectangular {
            columns, rows, hexes, index
        }
    }

    pub fn fetch<C: IntoCube>(&self, location: C) -> Result<&T, BadCoordinate> {
        let coordinate = location.cube()?;
        self.index
            .get(&coordinate)
            .ok_or_else(|| NoHexAtCoordinate::from(coordinate).into())
            .and_then(|i| Ok(&self.hexes[*i].1))
    }

    pub fn iter(&self) -> impl Iterator<Item = HexTile<T>> {
        self.hexes
            .iter()
            .map(|(c, d)| HexTile::new(c, d))
    }

    /// Will clone a copy of the `Rectangular<T>` grid and iterate through all hexagons
    /// applying the sent function/closure. Function takes a reference to the coordinate
    /// that the 
    pub fn fork_with<F: FnMut(&Cube, T) -> T>(&self, mut f: F) -> Self {
        let mut clone = self.clone();

        clone.hexes
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
        let output = self.hexes
            .iter()
            .fold((String::new(), 0, 0), |(output, col, row), (_cube, data)| {
                let mut output = format!("{}{} ", &output, data);
                let col = col + 1;

                // Check if we reach the end of the row
                let (col, row) = if col >= self.columns {
                    let row = row + 1;
                    let remainder = row % 2;
                    if remainder == 0 || (remainder != 0 && row >= self.rows) {
                        output = format!("{}\n", &output);
                    } else {
                        output = format!("{}\n  ", &output);
                    }
                    
                    (0, row)
                } else {
                    (col, row)
                };
                
                (output, col, row)
            });

        write!(f, "{}", &output.0)
    }
}

impl<T: Copy + Clone + Hash + PartialEq + Eq> Hash for Rectangular<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.columns.hash(state);
        self.rows.hash(state);
        self.hexes.hash(state);
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

        let f_grid = r_grid.fork_with(|_, h| h * 2);

        f_grid
            .iter()
            .for_each(|hex| {
                assert!(*hex.data() == 8);
            });
    }
}
