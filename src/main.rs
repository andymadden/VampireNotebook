mod core;

use crate::core::{GameCore, GameState};

fn main() {
    let game_core = GameCore::new(GameState::TitleScreen);

    game_core.run_game();
}
