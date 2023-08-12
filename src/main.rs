mod gamecore;
mod state;

use gamecore::GameCore;
use state::TitleState;

fn main() {
    let title_state = TitleState::new("Hey Gamer");
    let mut game_core = GameCore::new(&title_state);

    game_core.run_game();
}
