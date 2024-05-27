use rand::{thread_rng, Rng};
use termint::{
    enums::{bg::Bg, fg::Fg},
    geometry::constrain::Constrain,
    widgets::layout::Layout,
};

use crate::raw_span::RawSpan;

/// Struct representing 2048 board
pub struct Board {
    tiles: Vec<u16>,
    width: usize,
    height: usize,
}

impl Board {
    /// Creates new [`Board`]
    pub fn new(width: usize, height: usize) -> Self {
        let mut board = Self {
            // tiles: vec![0; width * height],
            tiles: vec![2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0],
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
        for _ in 0..self.height {
            let mut row = Layout::horizontal();
            for _ in 0..self.width {
                row.add_child(
                    self.get_tile(self.tiles[cur]),
                    Constrain::Length(6),
                );
                cur += 1;
            }
            layout.add_child(row, Constrain::Length(3));
        }

        layout
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
            while self.move_right(cur, cur - offset) {
                change = true;
            }
            cur += self.width;
        }
        if change {
            self.generate();
        }
    }

    pub fn left(&mut self) {
        let mut change = false;
        let mut cur = 0;
        let offset = self.width - 1;
        for _ in 0..self.height {
            while self.move_left(cur, cur + offset) {
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
        while self.tiles[pos] != 0 {
            pos = rng.gen_range(0..self.tiles.len());
        }

        self.tiles[pos] = 2;
    }

    fn move_up(&mut self, cur: usize) -> bool {
        let Some(next) = self.find_up_next(cur) else {
            return false;
        };

        let mut change = false;
        let cur_val = self.tiles[cur];
        if cur_val == 0 {
            self.tiles[cur] += self.tiles[next];
            self.tiles[next] = 0;
            self.move_up(cur);
            return true;
        }
        if cur_val == self.tiles[next] {
            self.tiles[cur] += self.tiles[next];
            self.tiles[next] = 0;
            change = true;
        }
        return self.move_up(cur + self.width) || change;
    }

    fn find_up_next(&self, mut cur: usize) -> Option<usize> {
        cur += self.width;
        while let Some(tile) = self.tiles.get(cur) {
            if *tile != 0 {
                return Some(cur);
            }
            cur += self.width;
        }
        None
    }

    fn move_down(&mut self, cur: usize) -> bool {
        let Some(next) = self.find_down_next(cur) else {
            return false;
        };

        let mut change = false;
        let cur_val = self.tiles[cur];
        if cur_val == 0 {
            self.tiles[cur] += self.tiles[next];
            self.tiles[next] = 0;
            self.move_down(cur);
            return true;
        }
        if cur_val == self.tiles[next] {
            self.tiles[cur] += self.tiles[next];
            self.tiles[next] = 0;
            change = true;
        }

        if cur < self.width {
            return change;
        }
        return self.move_down(cur - self.width) || change;
    }

    fn find_down_next(&self, mut cur: usize) -> Option<usize> {
        while cur >= self.width {
            cur -= self.width;
            if self.tiles[cur] != 0 {
                return Some(cur);
            }
        }
        None
    }

    fn move_right(&mut self, mut prev_id: usize, end: usize) -> bool {
        let mut change = false;
        while prev_id > end {
            if let Some(val) = self.tiles.get(prev_id - 1) {
                if *val != 0
                    && (self.tiles[prev_id] == 0
                        || self.tiles[prev_id] == *val)
                {
                    change = true;
                    self.tiles[prev_id] += *val;
                    self.tiles[prev_id - 1] = 0;
                }
            }
            prev_id -= 1;
        }
        change
    }

    fn move_left(&mut self, mut prev_id: usize, end: usize) -> bool {
        let mut change = false;
        while prev_id < end {
            if let Some(val) = self.tiles.get(prev_id + 1) {
                if *val != 0
                    && (self.tiles[prev_id] == 0
                        || self.tiles[prev_id] == *val)
                {
                    change = true;
                    self.tiles[prev_id] += *val;
                    self.tiles[prev_id + 1] = 0;
                }
            }
            prev_id += 1;
        }
        change
    }

    fn get_tile(&self, value: u16) -> Layout {
        if value == 0 {
            return self.get_empty_tile();
        }

        let lb = 0x797979;
        let db = self.get_tile_color(value);

        let mut vis = Layout::vertical();
        vis.add_child(
            RawSpan::new(" ▆▆▆▆▆").fg(Fg::Hex(db)).bg(Bg::Hex(lb)),
            Constrain::Length(1),
        );
        let pad = 5 - value.to_string().len();
        let pad_r = pad / 2;
        vis.add_child(
            RawSpan::new(format!(
                " {}{}{}{}",
                Bg::Hex(db),
                " ".repeat(pad - pad_r),
                value,
                " ".repeat(pad_r)
            ))
            .bg(Bg::Hex(lb)),
            Constrain::Length(1),
        );
        vis.add_child(
            RawSpan::new(format!(" {}▂▂▂▂▂", Bg::Hex(db)))
                .bg(Bg::Hex(lb))
                .fg(Fg::Hex(lb)),
            Constrain::Length(1),
        );
        vis
    }

    fn get_empty_tile(&self) -> Layout {
        let lb = 0x797979;
        let db = 0xbcbcbc;

        let mut vis = Layout::vertical();
        vis.add_child(
            RawSpan::new(" ▆▆▆▆▆").fg(Fg::Hex(db)).bg(Bg::Hex(lb)),
            Constrain::Length(1),
        );
        vis.add_child(
            RawSpan::new(format!(" {}     ", Bg::Hex(db))).bg(Bg::Hex(lb)),
            Constrain::Length(1),
        );
        vis.add_child(
            RawSpan::new(format!(" {}▂▂▂▂▂", Bg::Hex(db)))
                .bg(Bg::Hex(lb))
                .fg(Fg::Hex(lb)),
            Constrain::Length(1),
        );
        vis
    }

    fn get_tile_color(&self, value: u16) -> u32 {
        match value {
            2 => 0xeee4da,
            4 => 0xede0c8,
            8 => 0xf2b179,
            16 => 0xf59563,
            32 => 0xf67c5f,
            64 => 0xf65e3b,
            128 => 0xedcf72,
            256 => 0xedcc61,
            512 => 0xedc850,
            1024 => 0xedc53f,
            2048 => 0xedc22e,
            _ => 0x969696,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Self {
            tiles: vec![
                0, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 0, 0, 0, 0,
            ],
            width: 4,
            height: 4,
        };
        // board.generate();
        board
    }
}
