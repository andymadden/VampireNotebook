

use crate::GameState;
use std::io::{self, Write};

pub struct GameCore {
    pub state: GameState,
}

impl GameCore {
    pub fn new(state: GameState) -> GameCore {
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
                Ok(_) => self.process_input(&input),
                Err(e) => eprintln!("{}", e)
            }
        }
    }

    fn process_input(&self, input: &str) {
        println!("Processed: {}", input);
    }
}

