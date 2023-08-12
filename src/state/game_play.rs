use super::GameCore;
use super::GameState;

pub fn game_play_loop(gamecore: &mut GameCore) {
    println!("Game Play!");
    gamecore.set_state(GameState::PauseMenu);
}

