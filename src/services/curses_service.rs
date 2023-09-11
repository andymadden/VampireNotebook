use std::process::exit;

use pancurses::{Window, Input, endwin, echo};

pub use crate::screen::Screen;

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

    pub fn draw_screen(&self, screen: Screen) { // TODO: flesh this out to render a full main menu
       screen.draw(&self.window);
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
