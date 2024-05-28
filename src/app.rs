use std::{
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use termint::{
    geometry::constrain::Constrain,
    term::Term,
    widgets::{layout::Layout, spacer::Spacer},
};

use crate::{board::Board, error::Error, game_status::GameStatus};

pub struct App {
    board: Board,
    status: GameStatus,
    term: Term,
}

impl App {
    /// Creates new [`App`]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            board: Board::new(width, height),
            status: GameStatus::Playing,
            term: Term::new(),
        }
    }

    /// Runs the [`App`]
    pub fn run(&mut self) -> Result<(), Error> {
        // Saves screen, clears screen and hides cursor
        print!("\x1b[?1049h\x1b[2J\x1b[?25l");
        _ = stdout().flush();
        enable_raw_mode()?;

        let res = self.main_loop();

        disable_raw_mode()?;
        // Restores screen
        print!("\x1b[?1049l\x1b[?25h");
        _ = stdout().flush();

        res
    }

    /// Main loop of the [`App`]
    fn main_loop(&mut self) -> Result<(), Error> {
        self.render();
        loop {
            if poll(Duration::from_millis(100))? {
                self.key_listener()?;
            }
        }
    }

    /// Renders the [`App`]
    fn render(&self) {
        let mut wrapper = Layout::vertical().center();
        wrapper.add_child(self.render_status(), Constrain::Length(1));
        wrapper.add_child(
            self.board.get(),
            Constrain::Length(self.board.height()),
        );

        let mut main = Layout::horizontal().center();
        main.add_child(wrapper, Constrain::Length(self.board.width()));

        _ = self.term.render(main);
    }

    /// Handles key listening of the [`App`]
    fn key_listener(&mut self) -> Result<(), Error> {
        let Event::Key(KeyEvent { code, .. }) = read()? else {
            return Ok(());
        };

        match code {
            KeyCode::Up => self.status = self.board.up(),
            KeyCode::Down => self.status = self.board.down(),
            KeyCode::Left => self.status = self.board.left(),
            KeyCode::Right => self.status = self.board.right(),
            KeyCode::Char('r') => {
                self.board.reset();
                self.status = GameStatus::Playing;
                print!("\x1b[H\x1b[J");
            }
            KeyCode::Char('q') | KeyCode::Esc => return Err(Error::Exit),
            _ => return Ok(()),
        }

        self.render();
        Ok(())
    }

    fn render_status(&self) -> Layout {
        let mut status = Layout::horizontal();
        status.add_child(
            format!("Score: {}", self.board.score),
            Constrain::Min(0),
        );
        status.add_child(Spacer::new(), Constrain::Fill);
        status.add_child(self.status.to_string(), Constrain::Min(0));
        status
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            board: Default::default(),
            status: GameStatus::Playing,
            term: Term::new(),
        }
    }
}
