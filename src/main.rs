use app::App;
use args::Args;
use error::Error;
use termint::{enums::Color, widgets::StrSpanExtension};

mod app;
mod args;
mod board;
mod error;
mod game_status;
mod raw_span;

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", "Error".fg(Color::Red), e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let args = Args::parse(std::env::args())?;
    if args.help {
        Args::help();
        return Ok(());
    }

    let mut app = App::new(args.size, args.win);
    app.run()
}
