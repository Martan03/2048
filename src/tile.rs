use std::ops::AddAssign;

use termint::{
    enums::{bg::Bg, fg::Fg},
    geometry::constrain::Constrain,
    widgets::layout::Layout,
};

use crate::raw_span::RawSpan;

/// Represents tile in the 2048 board
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile(u16);

impl Tile {
    /// Creates new [`Tile`]
    pub fn new(value: u16) -> Self {
        Self(value)
    }

    /// Gets value of the [`Tile`]
    pub fn value(&self) -> u16 {
        self.0
    }

    /// Gets tile as termint widget
    pub fn get(&self) -> Layout {
        let db = self.color();

        let mut tile = Layout::vertical();
        tile.add_child(
            RawSpan::new(" ▆▆▆▆▆").fg(Fg::Hex(db)).bg(Bg::Hex(0x797979)),
            Constrain::Length(1),
        );

        tile.add_child(
            RawSpan::new(format!(" {}{}", Bg::Hex(db), self.value_str()))
                .bg(Bg::Hex(0x797979)),
            Constrain::Length(1),
        );

        tile.add_child(
            RawSpan::new(format!(" {}▂▂▂▂▂", Bg::Hex(db)))
                .bg(Bg::Hex(0x797979))
                .fg(Fg::Hex(0x797979)),
            Constrain::Length(1),
        );
        tile
    }

    /// Gets [`Tile`] color based on its value
    fn color(&self) -> u32 {
        match self.value() {
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

    /// Gets [`Tile`] value as string and centered so its length is 5
    fn value_str(&self) -> String {
        if self.value() == 0 {
            return "     ".to_string();
        }
        let pad = 5 - self.value().to_string().len();
        let pad_r = pad / 2;
        format!(
            "{}{}{}",
            " ".repeat(pad - pad_r),
            self.value(),
            " ".repeat(pad_r)
        )
    }
}

impl AddAssign for Tile {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl From<u16> for Tile {
    fn from(value: u16) -> Self {
        Tile(value)
    }
}

impl Into<u16> for Tile {
    fn into(self) -> u16 {
        self.0
    }
}
