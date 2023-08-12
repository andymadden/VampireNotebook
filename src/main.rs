mod gamecore;
mod state;

use gamecore::GameCore;
use state::TitleState;

fn main() {
    let game_core = GameCore::new(&TitleState {game_title: "Game Title"});

    game_core.run_game();
}
