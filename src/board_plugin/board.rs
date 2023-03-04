use std::fmt;

use bevy::prelude::*;

const NEIGBOURHOOD_OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(Resource)]
pub struct Board {
    tiles: Vec<(bool, Entity)>,
    pub width: usize,
    pub height: usize,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tiles = self.tiles();
        for x in 0..self.width {
            for y in 0..self.height {
                match self.get(x, y) {
                    true => write!(f, "X ({}) | ", tiles.alive_neighbors(x, y))?,
                    false => write!(f, "_ ({}) | ", tiles.alive_neighbors(x, y))?,
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            tiles: vec![(false, Entity::from_raw(0)); width * height],
            width,
            height,
        }
    }

    pub fn tiles(&self) -> Tiles {
        Tiles {
            tiles: self.tiles.iter().map(|t| t.0).collect(),
            width: self.width,
            height: self.height,
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.tiles[y * self.width + x].0
    }

    pub fn get_entity(&self, x: usize, y: usize) -> Entity {
        self.tiles[y * self.width + x].1
    }

    pub fn set_entity(&mut self, x: usize, y: usize, e: Entity) {
        self.tiles[y * self.width + x].1 = e
    }

    pub fn set(&mut self, x: usize, y: usize, v: bool) {
        self.tiles[y * self.width + x].0 = v
    }
}

#[test]
fn test_board() {
    let mut b = Board::new(4, 4);
    b.set(0, 0, true);
    b.set(0, 1, true);
    b.set(0, 2, true);
    // b.set(1, 1, true);
    // b.set(3, 3, true);
    // println!("{}", b.alive_neighbors(1, 1));
    println!("{}", b);
}

pub struct Tiles {
    tiles: Vec<bool>,
    width: usize,
    height: usize,
}

impl Tiles {
    pub fn get(&self, x: usize, y: usize) -> bool {
        self.tiles[y * self.width + x]
    }

    fn alive_neighbors(&self, x: usize, y: usize) -> usize {
        NEIGBOURHOOD_OFFSETS
            .iter()
            .map(|(x_d, y_d)| {
                (
                    (x as isize + x_d).rem_euclid(self.width as isize),
                    (y as isize + y_d).rem_euclid(self.height as isize),
                )
            })
            .filter(|(x_q, y_q)| self.get(*x_q as usize, *y_q as usize))
            .count()
    }

    pub fn step_cell(&self, x: usize, y: usize) -> bool {
        let n = self.alive_neighbors(x, y);
        match self.get(x, y) {
            true => {
                if n < 2 || n > 3 {
                    return false;
                }
                return true;
            }
            false => {
                if n == 3 {
                    return true;
                }
                return false;
            }
        }
    }
}
