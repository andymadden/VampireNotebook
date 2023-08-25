extern crate pancurses;

mod gamecore;
mod state;

use gamecore::GameCore;
use state::GameState;

fn main() {
    let mut game_core = GameCore::new(GameState::TitleScreen);
    game_core.run_game();
}
