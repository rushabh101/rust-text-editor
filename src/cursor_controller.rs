use std::cmp;
use crate::editor_rows::EditorRows;

pub struct CursorController {
    pub(crate) cursor_x: usize,
    pub(crate) cursor_y: usize,
    pub(crate) row_offset: usize,
}

impl CursorController {
    pub(crate) fn new() -> CursorController {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            row_offset: 0,
        }
    }

    pub(crate) fn move_cursor(&mut self, direction: char, editor_rows: EditorRows, win_size: (usize, usize)) {
        let screen_columns = win_size.0;
        let screen_rows = win_size.1;

        let number_of_rows = editor_rows.number_of_rows();
        match direction {
            'w' => {
                if self.cursor_y != 0 {
                    self.cursor_y -= 1;
                }
                else if self.row_offset != 0 {
                    self.row_offset -= 1;
                }
            }
            'a' => {
                if self.cursor_x != 0 {
                    self.cursor_x -= 1;
                }
            }
            's' => {
                if self.cursor_y != screen_rows - 1 {
                    self.cursor_y += 1;
                }
                else if self.row_offset < number_of_rows - 1{
                    self.row_offset += 1;
                }
            }
            'd' => {
                if self.cursor_x != screen_columns - 1 {
                    self.cursor_x += 1;
                }
            }
            _ => unimplemented!(),
        }

        let number_of_columns = editor_rows.get_row(self.cursor_y + self.row_offset).len();
        self.cursor_x = cmp::min(self.cursor_x, number_of_columns)
    }
}
