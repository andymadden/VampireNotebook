
use crate::state::GameState;
use std::io::{self, Write};

pub struct GameCore<'a> {
    pub state: &'a (dyn GameState + 'a),
}

impl<'a> GameCore<'a> {
    pub fn new(state: &'a dyn GameState) -> GameCore {
        GameCore { state }
    }

    pub fn run_game(&self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        let mut input: String;
        loop {
            input = String::new();
            print!(">");
            stdout.flush().unwrap();
            match stdin.read_line(&mut input) {
                Ok(_) => self.state.handle_loop(&input),
                Err(e) => eprintln!("{}", e)
            }
        }
    }

    fn process_input(&self, input: &str) {
        println!("Processed: {}", input);
    }
}

