use anyhow::Error;

mod chessboard;
mod chesspiece;
mod tui;

fn main() {
    color_eyre::install();
    let terminal = ratatui::init();
    tui::Game::new().run(terminal);
    ratatui::restore();
}
