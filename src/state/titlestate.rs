use crate::state::GameState;

pub struct TitleState {
    pub game_title: &'static str
}

impl GameState for TitleState {
    fn handle_loop(&self, input: &str) {
        println!("{}", &self.game_title);
    }
}
