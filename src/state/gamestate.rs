
pub trait GameState {
    fn handle_loop(&self, input: &str);
}
