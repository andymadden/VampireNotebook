use super::GameCore;
use super::GameState;

pub fn pause_menu_loop(gamecore: &mut GameCore) {
    println!("Pause Menu!");
    gamecore.set_state(GameState::TitleScreen);
}
