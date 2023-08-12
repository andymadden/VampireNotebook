
use crate::state::{GameState, Transition, title_screen_loop, main_menu_loop, game_play_loop, pause_menu_loop};

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
            match self.game_state {
                GameState::TitleScreen => title_screen_loop(self),
                GameState::MainMenu => main_menu_loop(self),
                GameState::GamePlay => game_play_loop(self),
                GameState::PauseMenu => pause_menu_loop(self)
            }

            match self.transition {
                Transition::Next(game_state) => {
                    self.game_state = game_state
                },
                Transition::Continue => ()
            }
        }
    }

    pub fn set_state(&mut self, game_state: GameState) {
        self.transition = Transition::Next(game_state);
    }
}

