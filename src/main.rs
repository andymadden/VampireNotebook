mod core;
mod models;

use crate::core::{GameCore, GameState};

fn main() {
    let game_core = GameCore { state: GameState::TitleScreen };

    game_core.run_game();
}
