use pancurses::Window;

use crate::services::data_service::{DataService, self};

mod character_creation;
mod main_menu;
mod new_npc;
mod new_resource;
mod new_skill;
mod title;

#[derive(Copy, Clone)]
pub enum Screen<'a> {
    Title(&'a DataService),
    MainMenu,
    CharacterCreation,
    NewNPC,
    NewSkill,
    NewResource,
}

impl<'a> Screen<'a> {
    pub fn draw(self, window: &Window) {
        match self {
            Screen::Title(data_service) => title::draw(window, data_service),
            Screen::MainMenu => main_menu::draw(window),
            Screen::CharacterCreation => character_creation::draw(window),
            Screen::NewNPC => new_npc::draw(window),
            Screen::NewSkill => new_skill::draw(window),
            Screen::NewResource => new_resource::draw(window),
        }
    }
}
