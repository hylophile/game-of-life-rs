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
    old_tiles: Vec<bool>,
    new_tiles: Vec<bool>,
    entities: Vec<Entity>,
    pub width: usize,
    pub height: usize,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in 0..self.width {
            for y in 0..self.height {
                match self.get(x, y) {
                    true => write!(f, "X ({}) | ", self.alive_neighbors(x, y))?,
                    false => write!(f, "_ ({}) | ", self.alive_neighbors(x, y))?,
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            old_tiles: vec![false; size],
            new_tiles: vec![false; size],
            entities: vec![Entity::from_raw(0); size],
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.old_tiles[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, v: bool) {
        self.new_tiles[y * self.width + x] = v
    }

    pub fn set_old(&mut self, x: usize, y: usize, v: bool) {
        self.old_tiles[y * self.width + x] = v
    }

    pub fn get_entity(&self, x: usize, y: usize) -> Entity {
        self.entities[y * self.width + x]
    }

    pub fn set_entity(&mut self, x: usize, y: usize, e: Entity) {
        self.entities[y * self.width + x] = e
    }

    pub fn step_board(&mut self) {
        std::mem::swap(&mut self.old_tiles, &mut self.new_tiles)
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
