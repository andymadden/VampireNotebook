
use std::io::{self, Write};

pub enum GameState {
    TitleScreen,
    CharacterCreation,
    GamePlay,
    Exiting,
}

pub struct GameCore {
    pub state: GameState,
}

impl GameCore {
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

    fn process_input(&self, input: &String) {
        println!("Processed: {}", input);
    }
}

