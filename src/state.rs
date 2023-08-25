mod game_play;
mod main_menu;
mod pause_menu;
mod title_state;

use super::GameCore;
use std::clone::Clone;
use std::marker::Copy;


#[derive(Copy, Clone)]
pub enum GameState {
    TitleScreen,
    MainMenu,
    GamePlay,
    PauseMenu
}

impl GameState { 
    pub fn init(self, core: &mut GameCore) {
        match self {
            GameState::TitleScreen => title_state::title_screen_init(core),
            GameState::MainMenu => main_menu::main_menu_init(core),
            GameState::GamePlay => game_play::game_play_init(core),
            GameState::PauseMenu => pause_menu::pause_menu_init(core),
        }
    }

    pub fn game_loop(self, core: &mut GameCore) {
        match self {
            GameState::TitleScreen => title_state::title_screen_loop(core),
            GameState::MainMenu => main_menu::main_menu_loop(core),
            GameState::GamePlay => game_play::game_play_loop(core),
            GameState::PauseMenu => pause_menu::pause_menu_loop(core),
        }
    }

    pub fn cleanup(self, core: &mut GameCore) {
        match self {
            GameState::TitleScreen => title_state::title_screen_cleanup(core),
            GameState::MainMenu => main_menu::main_menu_cleanup(core),
            GameState::GamePlay => game_play::game_play_cleanup(core),
            GameState::PauseMenu => pause_menu::pause_menu_cleanup(core),
        }
    }
}

pub enum Transition {
    Continue,
    Next(GameState)
}

