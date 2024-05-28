use rand::{thread_rng, Rng};
use termint::{
    enums::{bg::Bg, fg::Fg},
    geometry::constrain::Constrain,
    widgets::layout::Layout,
};

use crate::{raw_span::RawSpan, tile::Tile};

/// Struct representing 2048 board
pub struct Board {
    tiles: Vec<Tile>,
    pub score: usize,
    width: usize,
    height: usize,
}

impl Board {
    /// Creates new [`Board`]
    pub fn new(width: usize, height: usize) -> Self {
        let mut board = Self {
            tiles: vec![Tile::new(0); width * height],
            score: 0,
            width,
            height,
        };
        board.generate();
        board
    }

    /// Gets [`Board`] as termint widget
    pub fn get(&self) -> Layout {
        let mut layout = Layout::vertical();
        let mut cur = 0;

        layout.add_child(
            RawSpan::new("▂".repeat(6 * self.width + 1)).fg(Fg::Hex(0x797979)),
            Constrain::Length(1),
        );
        for _ in 0..self.height {
            let mut row = Layout::horizontal();
            for _ in 0..self.width {
                row.add_child(self.tiles[cur].get(), Constrain::Length(6));
                cur += 1;
            }
            row.add_child(self.get_right_border(), Constrain::Length(1));
            layout.add_child(row, Constrain::Length(3));
        }
        layout.add_child(
            RawSpan::new("▔".repeat(6 * self.width + 1)).fg(Fg::Hex(0x797979)),
            Constrain::Length(1),
        );

        layout
    }

    /// Gets width of the [`Board`]
    pub fn width(&self) -> usize {
        self.width * 6 + 1
    }

    /// Gets height of the [`Board`]
    pub fn height(&self) -> usize {
        self.height * 3 + 2
    }

    /// Moves [`Board`] tiles up
    pub fn up(&mut self) {
        let mut change = false;
        for i in 0..self.width {
            if self.move_up(i) {
                change = true;
            }
        }
        if change {
            self.generate();
        }
    }

    /// Moves [`Board`] tiles down
    pub fn down(&mut self) {
        let mut change = false;
        for i in (self.width * (self.height - 1))..self.tiles.len() {
            if self.move_down(i) {
                change = true;
            }
        }
        if change {
            self.generate();
        }
    }

    /// Moves [`Board`] tiles right
    pub fn right(&mut self) {
        let mut change = false;
        let mut cur = self.width - 1;
        let offset = cur;
        for _ in 0..self.height {
            if self.move_right(cur, cur - offset) {
                change = true;
            }
            cur += self.width;
        }
        if change {
            self.generate();
        }
    }

    /// Moves [`Board`] tiles left
    pub fn left(&mut self) {
        let mut change = false;
        let mut cur = 0;
        let offset = self.width - 1;
        for _ in 0..self.height {
            if self.move_left(cur, cur + offset) {
                change = true;
            }
            cur += self.width;
        }
        if change {
            self.generate();
        }
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
        return self.move_up(cur + self.width) || change;
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

        if cur < self.width {
            return change;
        }
        return self.move_down(cur - self.width) || change;
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
        cur += self.width;
        while let Some(tile) = self.tiles.get(cur) {
            if tile.value() != 0 {
                return Some(cur);
            }
            cur += self.width;
        }
        None
    }

    /// Finds next non-zero value down in column
    fn find_down_next(&self, mut cur: usize) -> Option<usize> {
        while cur >= self.width {
            cur -= self.width;
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

    /// Gets right border
    fn get_right_border(&self) -> Layout {
        let mut border = Layout::vertical();
        border.add_child(
            RawSpan::new(" ").bg(Bg::Hex(0x797979)),
            Constrain::Length(1),
        );
        border.add_child(
            RawSpan::new(" ").bg(Bg::Hex(0x797979)),
            Constrain::Length(1),
        );
        border.add_child(
            RawSpan::new(" ").bg(Bg::Hex(0x797979)),
            Constrain::Length(1),
        );
        border
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            tiles: vec![
                0.into(),
                2.into(),
                4.into(),
                8.into(),
                16.into(),
                32.into(),
                64.into(),
                128.into(),
                256.into(),
                512.into(),
                1024.into(),
                2048.into(),
                0.into(),
                0.into(),
                0.into(),
                0.into(),
            ],
            score: 0,
            width: 4,
            height: 4,
        }
    }
}
