
use super::state::{
    GameState,
    Transition,
};

pub struct GameCore {
    pub game_state: GameState,
    pub transition: Transition,
}

impl GameCore {
    pub fn new(state: GameState) -> GameCore {
        GameCore { game_state: state, transition: Transition::Continue }
    }

    pub fn run_game(&mut self) {
        loop {
            self.game_state.game_loop(self);

            match self.transition {
                Transition::Next(game_state) => {
                    self.game_state.cleanup(self);
                    self.game_state = game_state;
                    self.game_state.init(self);
                },
                Transition::Continue => ()
            }
        }
    }

    pub fn set_state(&mut self, game_state: GameState) {
        self.transition = Transition::Next(game_state);
    }
}

