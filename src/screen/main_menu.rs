use pancurses::Window;

use crate::util::usize_to_i32;

pub fn draw(window: &Window) {
    window.border('|', '|', '-', '-', '+', '+', '+', '+');

    window.refresh();
    let (max_y, max_x) = window.get_max_yx();

    let main_menu_content = "Placeholder content for main menu";

    window.mv(max_y / 2, max_x / 2 - usize_to_i32(main_menu_content.chars().count()) / 2);
    window.printw(main_menu_content);
    window.refresh();

    window.mv(0, 0);
}
