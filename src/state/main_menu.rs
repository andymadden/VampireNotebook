use super::GameCore;
use super::GameState;

pub fn main_menu_loop(gamecore: &mut GameCore) {
    println!("Main Menu!");
    gamecore.set_state(GameState::GamePlay);
}
