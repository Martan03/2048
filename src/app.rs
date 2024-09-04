use std::{
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use termint::{
    enums::{Color, Modifier},
    geometry::{Constraint, Coords, TextAlign},
    paragraph,
    term::Term,
    widgets::{Layout, Paragraph, Spacer, StrSpanExtension},
};

use crate::{board::Board, error::Error, game_status::GameStatus};

pub struct App {
    board: Board,
    status: GameStatus,
    term: Term,
}

impl App {
    /// Creates new [`App`]
    pub fn new(size: Coords, win: u16) -> Self {
        Self {
            board: Board::new(size, win),
            status: GameStatus::Playing,
            term: Term::new().small_screen(Self::small_screen()),
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

        match res {
            Err(Error::Exit) => Ok(()),
            _ => res,
        }
    }

    /// Main loop of the [`App`]
    fn main_loop(&mut self) -> Result<(), Error> {
        self.render()?;
        loop {
            if poll(Duration::from_millis(100))? {
                self.event()?;
            }
        }
    }

    /// Renders the [`App`]
    fn render(&mut self) -> Result<(), Error> {
        let mut game = Layout::vertical().center();
        game.add_child(self.render_status(), Constraint::Length(1));
        game.add_child(self.board.clone(), Constraint::Min(0));

        let mut wrapper = Layout::horizontal().center();
        wrapper.add_child(game, Constraint::Min(0));

        let mut main = Layout::vertical();
        main.add_child(wrapper, Constraint::Fill);
        main.add_child(self.render_help(), Constraint::Min(0));

        self.term.render(main)?;
        Ok(())
    }

    /// Handles key listening of the [`App`]
    fn event(&mut self) -> Result<(), Error> {
        match read()? {
            Event::Key(e) => self.key_handler(e),
            Event::Resize(_, _) => self.render(),
            _ => Ok(()),
        }
    }

    /// Renders the help screen
    fn render_help(&self) -> Paragraph {
        paragraph!(
            "[Arrows]Movement".fg(Color::Gray),
            "[r]Restart".fg(Color::Gray),
            "[Esc|q]Quit".fg(Color::Gray),
        )
        .separator(" ")
    }

    /// Handles key events
    fn key_handler(&mut self, event: KeyEvent) -> Result<(), Error> {
        match event.code {
            KeyCode::Up | KeyCode::Char('k') => self.status = self.board.up(),
            KeyCode::Down | KeyCode::Char('j') => {
                self.status = self.board.down()
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.status = self.board.left()
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.status = self.board.right()
            }
            KeyCode::Char('r') => {
                self.board.reset();
                self.status = GameStatus::Playing;
            }
            KeyCode::Char('q') | KeyCode::Esc => return Err(Error::Exit),
            _ => return Ok(()),
        }
        self.render()
    }

    fn render_status(&self) -> Layout {
        let mut status = Layout::horizontal();
        status.add_child(
            format!("Score: {}", self.board.score),
            Constraint::Min(0),
        );
        status.add_child(Spacer::new(), Constraint::Fill);
        status.add_child(self.status.to_string(), Constraint::Min(0));
        status
    }

    /// Small screen to be displayed, when game can't fit
    fn small_screen() -> Layout {
        let mut layout = Layout::vertical().center();
        layout.add_child(
            "Terminal too small!"
                .modifier(Modifier::BOLD)
                .align(TextAlign::Center),
            Constraint::Min(0),
        );
        layout.add_child(
            "You have to increase terminal size".align(TextAlign::Center),
            Constraint::Min(0),
        );
        layout
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
