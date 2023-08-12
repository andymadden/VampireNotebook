
use crate::state::GameState;

pub struct GameCore<'a> {
    pub state: &'a (dyn GameState + 'a),
}

impl<'a> GameCore<'a> {
    pub fn new(state: &'a dyn GameState) -> GameCore {
        GameCore { state }
    }

    pub fn run_game(&mut self) {
        loop {
            self.state.game_loop();

            match self.state.get_next_state() {
                Some(next_state) => {
                    self.state = next_state;
                },
                None => ()
            }
        }
    }

    fn process_input(&self, input: &str) {
        println!("Processed: {}", input);
    }
}

