use app::App;

mod app;
mod board;
mod error;

fn main() {
    let mut app = App::default();
    _ = app.run();
}
