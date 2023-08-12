
pub trait GameState {
    fn game_loop(&self);
    fn get_next_state(&self) -> Option<&dyn GameState>;
}

pub struct TitleState<'a> {
    pub game_title: &'static str,
    next_state: Option<&'a dyn GameState>
}

impl<'a> GameState for TitleState<'a> {
    fn game_loop(&self) {
        
    }

    fn get_next_state(&self) -> Option<&dyn GameState> {
        self.next_state
    }
}

impl<'a> TitleState<'a> {
    pub fn new(title: &'static str) -> TitleState<'_> {
        TitleState { game_title: title, next_state: None }
    }
}
