mod title_state;
mod main_menu;
mod game_play;
mod pause_menu;

use std::marker::Copy;
use std::clone::Clone;

use crate::gamecore::GameCore;

#[derive(Copy, Clone)]
pub enum GameState {
    TitleScreen,
    MainMenu,
    GamePlay,
    PauseMenu
}

pub enum Transition {
    Continue,
    Next(GameState)
}

pub use title_state::title_screen_loop;
pub use main_menu::main_menu_loop;
pub use game_play::game_play_loop;
pub use pause_menu::pause_menu_loop;

