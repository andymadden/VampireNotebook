
use std::process::exit;

use pancurses::{Window, Input, endwin, echo};
use crate::util::{self, usize_to_i32};

pub struct CursesService<'a> {
    window: &'a mut Window
}

impl<'a> CursesService<'a> { 
    // TODO: SCREEN - Main Menu
    // TODO: SCREEN - Title Screen
    // TODO: SCREEN - Player Character Creation
    // TODO: SCREEN - Prompt Response
    // TODO: COMPONENT - ASCII Canvas (for displaying ascii art)
    // TODO: COMPONENT - TextBox
    // TODO: COMPONENT - Header
    // TODO: COMPONENT - Prompt Display
    // TODO: COMPONENT - Story Character Selector
    // TODO: COMPONENT - Player Skill Selector
    // TODO: COMPONENT - Player Resource Selector
    // TODO: COMPONENT - Player Memory Selector

    pub fn new(window: &'a mut Window) -> Self {
        CursesService { window }
    }

    pub fn draw_border(&self) -> () {
        self.window.border('|', '|', '-', '-', '+', '+', '+', '+');
    }

    pub fn draw_header(&self, text: &str) -> () {
        self.window.refresh();
        let rows = text.split("\n");

        let row_count: i32 = util::usize_to_i32(rows.clone().count());

        let mut max_row_length = 0;
        for row in rows.clone() {
            if row.chars().count() > max_row_length {
                max_row_length = row.chars().count();
            }
        }

        let max_row_length: i32 = util::usize_to_i32(max_row_length);

        let (max_y, max_x) = self.window.get_max_yx();
        let root_y = max_y / 2 - row_count / 2;
        let root_x = max_x / 2 - max_row_length / 2;
        self.window.mv(root_y, root_x);
        for row in rows.clone() {
            let (cur_y, _) = self.window.get_cur_yx();
            self.window.printw(row);
            self.window.mv(cur_y+1, root_x);
        }

        let instruction_text = "[Press any key to begin]";
        let instruction_text_len: i32 = util::usize_to_i32(instruction_text.chars().count());
        self.window.mv(root_y + row_count + 1, max_x / 2 - instruction_text_len / 2);
        self.window.printw(instruction_text);
    }

    pub fn draw_main_menu(&self) { // TODO: flesh this out to render a full main menu
        self.window.refresh();
        let (max_y, max_x) = self.window.get_max_yx();
    
        let main_menu_content = "Placeholder content for main menu";

        self.window.mv(max_y / 2, max_x / 2 - usize_to_i32(main_menu_content.chars().count()) / 2);
        self.window.printw(main_menu_content);
        self.window.refresh();

        self.window.mv(0, 0);
    }

    pub fn get_input(&self) -> Option<Input> {
        self.window.getch()
    }

    pub fn clear(&self) {
        self.window.clear();
    }

    pub fn destroy(&self) {
        echo();
        endwin();
        exit(0);
    }
}
