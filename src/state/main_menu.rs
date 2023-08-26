use std::process::exit;

use super::GameCore;
use super::GameState;

pub fn main_menu_init(gamecore: &mut GameCore) {

}

pub fn main_menu_loop(gamecore: &mut GameCore) {
    gamecore.curses_service.draw_border();

    gamecore.curses_service.draw_main_menu();

    gamecore.curses_service.get_input();
    gamecore.curses_service.destroy();
}

pub fn main_menu_cleanup(gamecore: &mut GameCore) {
    gamecore.curses_service.clear();
}
