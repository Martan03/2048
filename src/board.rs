use rand::{thread_rng, Rng};
use termint::{geometry::constrain::Constrain, widgets::layout::Layout};

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
            tiles: vec![0; width * height],
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
                    self.tiles[cur].to_string(),
                    Constrain::Length(5),
                );
                cur += 1;
            }
            layout.add_child(row, Constrain::Length(1));
        }

        layout
    }

    /// Moves [`Board`] tiles up
    pub fn up(&mut self) {
        let mut change = false;
        for i in 0..self.width {
            while self.move_up(i) {
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
            while self.move_down(i) {
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

    fn move_up(&mut self, mut prev_id: usize) -> bool {
        let mut change = false;
        while let Some(val) = self.tiles.get(prev_id + self.width) {
            if *val != 0
                && (self.tiles[prev_id] == 0 || self.tiles[prev_id] == *val)
            {
                change = true;
                self.tiles[prev_id] += *val;
                self.tiles[prev_id + self.width] = 0;
            }
            prev_id += self.width;
        }
        change
    }

    fn move_down(&mut self, mut prev_id: usize) -> bool {
        let mut change = false;
        while prev_id >= self.width {
            if let Some(val) = self.tiles.get(prev_id - self.width) {
                if *val != 0
                    && (self.tiles[prev_id] == 0
                        || self.tiles[prev_id] == *val)
                {
                    change = true;
                    self.tiles[prev_id] += self.tiles[prev_id - self.width];
                    self.tiles[prev_id - self.width] = 0;
                }
            }
            prev_id -= self.width;
        }
        change
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
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Self {
            tiles: vec![0; 16],
            width: 4,
            height: 4,
        };
        board.generate();
        board
    }
}
