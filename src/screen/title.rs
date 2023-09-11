use pancurses::Window;
use crate::util::usize_to_i32;
use crate::services::data_service::DataService;

pub fn draw(window: &Window, data_service: &DataService) {
    window.border('|', '|', '-', '-', '+', '+', '+', '+');

    window.refresh();
    let rows = data_service.get_header().split("\n");

    let row_count: i32 = usize_to_i32(rows.clone().count());

    let mut max_row_length = 0;
    for row in rows.clone() {
        if row.chars().count() > max_row_length {
            max_row_length = row.chars().count();
        }
    }

    let max_row_length: i32 = usize_to_i32(max_row_length);

    let (max_y, max_x) = window.get_max_yx();
    let root_y = max_y / 2 - row_count / 2;
    let root_x = max_x / 2 - max_row_length / 2;
    window.mv(root_y, root_x);
    for row in rows.clone() {
        let (cur_y, _) = window.get_cur_yx();
        window.printw(row);
        window.mv(cur_y+1, root_x);
    }

    let instruction_text = "[Press any key to begin]";
    let instruction_text_len: i32 = usize_to_i32(instruction_text.chars().count());
    window.mv(root_y + row_count + 1, max_x / 2 - instruction_text_len / 2);
    window.printw(instruction_text);

}
