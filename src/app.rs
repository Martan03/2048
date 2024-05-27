use std::{
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use termint::term::Term;

use crate::{board::Board, error::Error};

pub struct App {
    board: Board,
    term: Term,
}

impl App {
    /// Creates new [`App`]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            board: Board::new(width, height),
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

    fn render(&self) {
        _ = self.term.render(self.board.get());
    }

    fn key_listener(&mut self) -> Result<(), Error> {
        let Event::Key(KeyEvent { code, .. }) = read()? else {
            return Ok(());
        };

        match code {
            KeyCode::Up => self.board.up(),
            KeyCode::Down => self.board.down(),
            KeyCode::Left => self.board.left(),
            KeyCode::Right => self.board.right(),
            KeyCode::Char('q') | KeyCode::Esc => return Err(Error::Exit),
            _ => return Ok(()),
        }

        self.render();
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            board: Default::default(),
            term: Term::new(),
        }
    }
}
