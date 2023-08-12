use super::GameCore;
use super::GameState;

pub fn title_screen_loop(gamecore: &mut GameCore) {
    println!("Title Screen!");
    gamecore.set_state(GameState::MainMenu);
}
