use super::GameCore;
use super::GameState;

pub fn main_menu_init(gamecore: &mut GameCore) {

}

pub fn main_menu_loop(gamecore: &mut GameCore) {
    println!("Main Menu!");
    gamecore.set_state(GameState::GamePlay);
}

pub fn main_menu_cleanup(gamecore: &mut GameCore) {

}
