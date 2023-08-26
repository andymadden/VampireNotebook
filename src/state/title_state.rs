use super::GameCore;

pub fn title_screen_init(gamecore: &mut GameCore) {
    
}

pub fn title_screen_loop(gamecore: &mut GameCore) {
    gamecore.curses_service.draw_border();

    gamecore.curses_service.draw_header(gamecore.data_service.get_header());
    gamecore.curses_service.get_input();
}

pub fn title_screen_cleanup(gamecore: &mut GameCore) {
    gamecore.curses_service.clear();
}

