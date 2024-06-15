use std::{cmp, io};
use crossterm::{cursor, execute, queue, terminal};
use std::io::{stdout, Write};
use crossterm::terminal::ClearType;
use crate::editor_rows::EditorRows;
use crate::cursor_controller::CursorController;
use crate::editor_contents::EditorContents;

pub struct Output {
    editor_contents: EditorContents,
    cursor_controller: CursorController,
    editor_rows: EditorRows,
}

impl Output {

    pub(crate) fn new() -> Self {

        Self {
            editor_contents: EditorContents::new(),
            cursor_controller: CursorController::new(),
            editor_rows: EditorRows::new(),
        }
    }

    pub(crate) fn clear_screen() -> Result<(), io::Error>{
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    fn draw_rows(&mut self, win_size: (usize, usize)) {

        let screen_rows = win_size.1;
        let screen_columns = win_size.0;

        for i in 0..screen_rows {
            let file_row = i + self.cursor_controller.row_offset;
            if file_row >= self.editor_rows.number_of_rows() {
                self.editor_contents.push('~');
            }
            else {
                let len = cmp::min(self.editor_rows.get_row(file_row).len(), screen_columns);
                self.editor_contents.push_str(&self.editor_rows.get_row(file_row)[..len])
            }

            queue!(
                self.editor_contents,
                terminal::Clear(ClearType::UntilNewLine)
            )
                .unwrap();
            if i < screen_rows - 1 {
                self.editor_contents.push_str("\r\n");
            }
        }
    }

    pub(crate) fn refresh_screen(&mut self, win_size: (usize, usize)) -> Result<(), io::Error> {
        queue!(self.editor_contents, cursor::Hide, cursor::MoveTo(0, 0))?;
        self.draw_rows(win_size);
        /* modify */
        let cursor_x = self.cursor_controller.cursor_x;
        let cursor_y = self.cursor_controller.cursor_y;
        queue!(
            self.editor_contents,
            cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            cursor::Show
        )?;
        /* end */
        self.editor_contents.flush()
    }

    pub(crate) fn move_cursor(&mut self, direction:char, win_size: (usize, usize)) {
        let text_limits = (self.editor_rows.number_of_rows(), self.editor_rows.get_row(self.cursor_controller.cursor_y + self.cursor_controller.row_offset).len());
        self.cursor_controller.move_cursor(direction, self.editor_rows.clone(), win_size);
    }
}
