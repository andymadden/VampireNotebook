use super::{GameCore, Transition, GameState};
use crate::Screen;

pub fn title_screen_init(gamecore: &mut GameCore) {
    
}

pub fn title_screen_loop(gamecore: &mut GameCore) {
    gamecore.curses_service.draw_border();

    gamecore.curses_service.draw_screen(Screen::Title(&gamecore.data_service));

    gamecore.curses_service.get_input();
    gamecore.set_state(GameState::MainMenu);
}

pub fn title_screen_cleanup(gamecore: &mut GameCore) {
    gamecore.curses_service.clear();
}

