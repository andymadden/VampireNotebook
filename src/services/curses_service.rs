
use pancurses::{Window, Input};

pub struct CursesService<'a> {
    window: &'a mut Window
}

impl<'a> CursesService<'a> {
    pub fn new(window: &'a mut Window) -> Self {
        CursesService { window }
    }

    pub fn draw_border(&self) -> () {
        self.window.border('|', '|', '-', '-', '+', '+', '+', '+');
    }

    pub fn draw_header(&self, text: &str) -> () {
        self.window.refresh();
        let rows = text.split("\n");

        let row_count: i32 = match rows.clone().count().try_into() {
            Ok(result) => result,
            Err(_) => panic!("Error converting usize into i32 when getting header row count")
        };

        let mut max_row_length = 0;
        for row in rows.clone() {
            if row.len() > max_row_length {
                max_row_length = row.to_string().len();
            }
        }

        let max_row_length: i32 = match max_row_length.try_into() {
            Ok(result) => result,
            Err(_) => panic!("Could not convert max row length into i32")
        };

        let (max_y, max_x) = self.window.get_max_yx();
        let root_y = max_y / 2 - row_count / 2;
        let root_x = max_x / 2 - max_row_length / 2;
        self.window.mv(root_y, root_x);
        self.window.printw(format!("{}, {} => {}, {}", max_y, max_x, root_y, root_x));
        self.window.mv(0, 0);
    }

    pub fn get_input(&self) -> Option<Input> {
        self.window.getch()
    }

    pub fn clear(&self) {
        self.window.clear();
    }
}
