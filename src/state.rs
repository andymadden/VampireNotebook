
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

pub fn title_screen_loop(gamecore: &mut GameCore) {
    println!("Title Screen!");
    gamecore.set_state(GameState::MainMenu);
}

pub fn main_menu_loop(gamecore: &mut GameCore) {
    println!("Main Menu!");
    gamecore.set_state(GameState::GamePlay);
}

pub fn game_play_loop(gamecore: &mut GameCore) {
    println!("Game Play!");
    gamecore.set_state(GameState::PauseMenu);
}

pub fn pause_menu_loop(gamecore: &mut GameCore) {
    println!("Pause Menu!");
    gamecore.set_state(GameState::TitleScreen);
}
