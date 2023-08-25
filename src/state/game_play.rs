use super::GameCore;
use super::GameState;

pub fn game_play_init(gamecore: &mut GameCore) {

}

pub fn game_play_loop(gamecore: &mut GameCore) {
    println!("Game Play!");
    gamecore.set_state(GameState::PauseMenu);
}

pub fn game_play_cleanup(gamecore: &mut GameCore) {
    
}
