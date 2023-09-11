extern crate pancurses;

mod gamecore;
mod screen;
mod services;
mod state;
mod util;
mod widget;

use gamecore::GameCore;
use pancurses::{initscr, noecho};
use state::GameState;

fn main() {
    let mut window = initscr();
    noecho();

    let mut game_core = GameCore::new(GameState::TitleScreen, &mut window);
    game_core.run_game();
}
