use super::GameCore;
use super::GameState;

pub fn pause_menu_init(gamecore: &mut GameCore) {
    
}

pub fn pause_menu_loop(gamecore: &mut GameCore) {
    println!("Pause Menu!");
    gamecore.set_state(GameState::TitleScreen);
}

pub fn pause_menu_cleanup(gamecore: &mut GameCore) {

}
