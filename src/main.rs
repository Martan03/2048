use app::App;

mod app;
mod board;
mod error;
mod game_status;
mod raw_span;
mod tile;

fn main() {
    let mut app = App::new(4, 4);
    _ = app.run();
}
