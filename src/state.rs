mod title_state;
mod main_menu;
mod game_play;
mod pause_menu;

use std::marker::Copy;
use std::clone::Clone;

use title_state::{
    title_screen_init,
    title_screen_loop,
    title_screen_cleanup,
};
use main_menu::{
    main_menu_init,
    main_menu_loop,
    main_menu_cleanup,
};
use game_play::{
    game_play_init,
    game_play_loop,
    game_play_cleanup,
};
use pause_menu::{
    pause_menu_init,
    pause_menu_loop,
    pause_menu_cleanup,
};

use crate::gamecore::GameCore;

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
            GameState::TitleScreen => title_screen_init(core),
            GameState::MainMenu => main_menu_init(core),
            GameState::GamePlay => game_play_init(core),
            GameState::PauseMenu => pause_menu_init(core),
        }       
    }

    pub fn game_loop(self, core: &mut GameCore) {
        match self {
            GameState::TitleScreen => title_screen_loop(core),
            GameState::MainMenu => main_menu_loop(core),
            GameState::GamePlay => game_play_loop(core),
            GameState::PauseMenu => pause_menu_loop(core),
        }
    }

    pub fn cleanup(self, core: &mut GameCore) {
        match self {
            GameState::TitleScreen => title_screen_cleanup(core),
            GameState::MainMenu => main_menu_cleanup(core),
            GameState::GamePlay => game_play_cleanup(core),
            GameState::PauseMenu => pause_menu_cleanup(core),
        }
    }
}

pub enum Transition {
    Continue,
    Next(GameState)
}

