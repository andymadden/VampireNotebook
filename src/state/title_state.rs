use super::GameCore;
use super::GameState;

pub fn title_screen_init(gamecore: &mut GameCore) {
    
}

pub fn title_screen_loop(gamecore: &mut GameCore) {
    println!("Title Screen!");
    gamecore.set_state(GameState::MainMenu);
}

pub fn title_screen_cleanup(gamecore: &mut GameCore) {

}
