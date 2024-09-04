use rand::{thread_rng, Rng};
use termint::{geometry::Coords, widgets::Widget};
use tile::Tile;

use crate::game_status::GameStatus;

mod tile;
mod tui;

/// Struct representing 2048 board
#[derive(Debug, Clone)]
pub struct Board {
    pub tiles: Vec<Tile>,
    pub score: usize,
    pub size: Coords,
    pub win: u16,
}

impl Board {
    /// Creates new [`Board`]
    pub fn new(size: Coords, win: u16) -> Self {
        let mut board = Self {
            tiles: vec![Tile::new(0); size.x * size.y],
            score: 0,
            size,
            win,
        };
        board.generate();
        board
    }

    /// Resets the [`Board`]
    pub fn reset(&mut self) {
        self.tiles = vec![Tile::new(0); self.size.x * self.size.y];
        self.score = 0;
        self.generate();
    }

    /// Moves [`Board`] tiles up
    pub fn up(&mut self) -> GameStatus {
        let mut change = false;
        for i in 0..self.size.x {
            if self.move_up(i) {
                change = true;
            }
        }
        if change {
            self.generate();
        }
        self.status()
    }

    /// Moves [`Board`] tiles down
    pub fn down(&mut self) -> GameStatus {
        let mut change = false;
        for i in (self.size.x * (self.size.y - 1))..self.tiles.len() {
            if self.move_down(i) {
                change = true;
            }
        }
        if change {
            self.generate();
        }
        self.status()
    }

    /// Moves [`Board`] tiles right
    pub fn right(&mut self) -> GameStatus {
        let mut change = false;
        let mut cur = self.size.x - 1;
        let offset = cur;
        for _ in 0..self.size.y {
            if self.move_right(cur, cur - offset) {
                change = true;
            }
            cur += self.size.x;
        }
        if change {
            self.generate();
        }
        self.status()
    }

    /// Moves [`Board`] tiles left
    pub fn left(&mut self) -> GameStatus {
        let mut change = false;
        let mut cur = 0;
        let offset = self.size.x - 1;
        for _ in 0..self.size.y {
            if self.move_left(cur, cur + offset) {
                change = true;
            }
            cur += self.size.x;
        }
        if change {
            self.generate();
        }
        self.status()
    }

    /// Generates new tile in empty space of [`Board`]
    fn generate(&mut self) {
        let mut rng = thread_rng();

        let mut pos = rng.gen_range(0..self.tiles.len());
        while self.tiles[pos].value() != 0 {
            pos = rng.gen_range(0..self.tiles.len());
        }

        let rng_val = rng.gen_range(0..10);
        if rng_val == 9 {
            self.tiles[pos] = 4.into();
        } else {
            self.tiles[pos] = 2.into();
        }
    }

    /// Gets status of the game
    fn status(&self) -> GameStatus {
        if self.check_victory() {
            GameStatus::Victory
        } else if self.check_full() {
            GameStatus::GameOver
        } else {
            GameStatus::Playing
        }
    }

    /// Checks for the victory
    fn check_victory(&self) -> bool {
        for tile in self.tiles.iter() {
            if tile.value() == self.win {
                return true;
            }
        }
        false
    }

    /// Checks whether [`Board`] is full and no tiles can be joined
    fn check_full(&self) -> bool {
        for y in 0..self.size.y {
            let offset = y * self.size.x;
            for x in 0..self.size.x {
                if self.tiles[offset + x].value() == 0 {
                    return false;
                }

                if let Some(tile) = self.tiles.get(offset + x + 1) {
                    if x != self.size.x - 1 && *tile == self.tiles[offset + x]
                    {
                        return false;
                    }
                }
                if let Some(tile) = self.tiles.get(offset + x + self.size.x) {
                    if *tile == self.tiles[offset + x] {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Moves tile from given position to given position
    fn move_tile(&mut self, to: usize, from: usize) {
        let from_val = self.tiles[from];
        self.tiles[to] += from_val;
        self.tiles[from] = 0.into();
    }

    /// Moves column up
    fn move_up(&mut self, cur: usize) -> bool {
        let Some(next) = self.find_up_next(cur) else {
            return false;
        };

        let mut change = false;
        let cur_val = self.tiles[cur];
        if cur_val.value() == 0 {
            self.move_tile(cur, next);
            self.move_up(cur);
            return true;
        }
        if cur_val == self.tiles[next] {
            self.move_tile(cur, next);
            self.score += self.tiles[cur].value() as usize;
            change = true;
        }
        return self.move_up(cur + self.size.x) || change;
    }

    /// Moves column down
    fn move_down(&mut self, cur: usize) -> bool {
        let Some(next) = self.find_down_next(cur) else {
            return false;
        };

        let mut change = false;
        let cur_val = self.tiles[cur];
        if cur_val.value() == 0 {
            self.move_tile(cur, next);
            self.move_down(cur);
            return true;
        }
        if cur_val == self.tiles[next] {
            self.move_tile(cur, next);
            self.score += self.tiles[cur].value() as usize;
            change = true;
        }

        if cur < self.size.x {
            return change;
        }
        return self.move_down(cur - self.size.x) || change;
    }

    /// Moves row right
    fn move_right(&mut self, cur: usize, end: usize) -> bool {
        let Some(next) = self.find_right_next(cur, end) else {
            return false;
        };

        let mut change = false;
        let cur_val = self.tiles[cur];
        if cur_val.value() == 0 {
            self.move_tile(cur, next);
            self.move_right(cur, end);
            return true;
        }
        if cur_val == self.tiles[next] {
            self.move_tile(cur, next);
            self.score += self.tiles[cur].value() as usize;
            change = true;
        }

        if cur < end {
            return change;
        }
        return self.move_right(cur - 1, end) || change;
    }

    /// Moves row left
    fn move_left(&mut self, cur: usize, end: usize) -> bool {
        let Some(next) = self.find_left_next(cur, end) else {
            return false;
        };

        let mut change = false;
        let cur_val = self.tiles[cur];
        if cur_val.value() == 0 {
            self.move_tile(cur, next);
            self.move_left(cur, end);
            return true;
        }
        if cur_val == self.tiles[next] {
            self.move_tile(cur, next);
            self.score += self.tiles[cur].value() as usize;
            change = true;
        }

        if cur > end {
            return change;
        }
        return self.move_left(cur + 1, end) || change;
    }

    /// Finds next non-zero value up in column
    fn find_up_next(&self, mut cur: usize) -> Option<usize> {
        cur += self.size.x;
        while let Some(tile) = self.tiles.get(cur) {
            if tile.value() != 0 {
                return Some(cur);
            }
            cur += self.size.x;
        }
        None
    }

    /// Finds next non-zero value down in column
    fn find_down_next(&self, mut cur: usize) -> Option<usize> {
        while cur >= self.size.x {
            cur -= self.size.x;
            if self.tiles[cur].value() != 0 {
                return Some(cur);
            }
        }
        None
    }

    /// Finds next non-zero value left in row
    fn find_left_next(&self, mut cur: usize, end: usize) -> Option<usize> {
        while cur < end {
            cur += 1;
            if self.tiles[cur].value() != 0 {
                return Some(cur);
            }
        }
        None
    }

    /// Finds next non-zero value right in row
    fn find_right_next(&self, mut cur: usize, end: usize) -> Option<usize> {
        while cur > end {
            cur -= 1;
            if self.tiles[cur].value() != 0 {
                return Some(cur);
            }
        }
        None
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            tiles: vec![Tile::new(0); 16],
            score: 0,
            size: Coords::new(4, 4),
            win: 2048,
        }
    }
}

impl Into<Box<dyn Widget>> for Board {
    fn into(self) -> Box<dyn Widget> {
        Box::new(self)
    }
}
