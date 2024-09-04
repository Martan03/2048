use termint::{
    buffer::Buffer,
    enums::Color,
    geometry::{Constraint, Coords, Unit},
    widgets::{Grid, Layout, Widget},
};

use crate::{board::Board, raw_span::RawSpan};

impl Widget for Board {
    fn render(&self, buffer: &mut Buffer) {
        let mut grid = Grid::new(
            vec![Unit::Length(6); self.size.x],
            vec![Unit::Length(3); self.size.y],
        );
        grid.col(Unit::Length(1));

        let mut cur = 0;
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                grid.add_child(self.tiles[cur].clone(), x, y);
                cur += 1;
            }
            grid.add_child(self.get_right_border(), self.size.x, y);
        }

        let mut layout = Layout::vertical();
        layout
            .add_child(Box::new(grid) as Box<dyn Widget>, Constraint::Min(0));
        layout.add_child(
            RawSpan::new("▔".repeat(6 * self.size.x + 1))
                .fg(Color::Hex(0x797979)),
            Constraint::Length(1),
        );
        layout.render(buffer);
    }

    fn height(&self, _size: &Coords) -> usize {
        self.size.y * 3 + 2
    }

    fn width(&self, _size: &Coords) -> usize {
        self.size.x * 6 + 1
    }
}

impl Board {
    /// Gets right border
    fn get_right_border(&self) -> Layout {
        let mut border = Layout::vertical();
        border.add_child(
            RawSpan::new(" ").bg(Color::Hex(0x797979)),
            Constraint::Length(1),
        );
        border.add_child(
            RawSpan::new(" ").bg(Color::Hex(0x797979)),
            Constraint::Length(1),
        );
        border.add_child(
            RawSpan::new(" ").bg(Color::Hex(0x797979)),
            Constraint::Length(1),
        );
        border
    }
}
