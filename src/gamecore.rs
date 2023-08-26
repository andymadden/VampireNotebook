
use pancurses::Window;

use crate::services::data_service::DataService;

use super::state::{GameState, Transition};
use super::services::curses_service::CursesService;

pub struct GameCore<'a> {
    pub game_state: GameState,
    pub state_transition: Transition,
    pub curses_service: CursesService<'a>,
    pub data_service: DataService,
}

impl<'a> GameCore<'a> {
    pub fn new(state: GameState, window: &'a mut Window) -> GameCore {
        GameCore {
            game_state: state, state_transition: Transition::Continue,
            curses_service: CursesService::new(window), data_service: DataService::new(),
        }
    }

    pub fn run_game(&mut self) {
        loop {
            self.game_state.game_loop(self);

            match self.state_transition {
                Transition::Next(game_state) => {
                    self.game_state.cleanup(self);
                    self.game_state = game_state;
                    self.game_state.init(self);
                    self.state_transition = Transition::Continue;
                },
                Transition::Continue => ()
            }
        }
    }

    pub fn set_state(&mut self, game_state: GameState) {
        self.state_transition = Transition::Next(game_state);
    }
}

