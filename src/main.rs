use crossterm::{cursor, event, execute, queue, terminal};
use std::io::{Read, Write};
use std::io::stdout;
use std::time::Duration;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal::ClearType;
use std::{cmp, env, fs, io};
use std::path::Path;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
        Output::clear_screen().expect("Error");
    }
}

struct Reader;
impl Reader {
    fn read_key(&self) -> Result<KeyEvent, io::Error> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }
}

struct Editor {
    reader: Reader,
    output: Output,
}
impl Editor {
    fn new() -> Self {
        Self {
            reader: Reader,
            output: Output::new(),
        }
    }

    fn process_keypress(&mut self, win_size: (usize, usize)) -> Result<bool, io::Error> {
        match self.reader.read_key()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } => return Ok(false),

            KeyEvent {
                code: KeyCode::Char(val @ ('w' | 'a' | 's' | 'd')),
                modifiers: event::KeyModifiers::NONE,
                kind: event::KeyEventKind::Press,
                ..
            } => self.output.move_cursor(val, win_size),
            _ => {}
        }
        Ok(true)
    }

    fn run(&mut self) -> Result<bool, io::Error> {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();

        self.output.refresh_screen(win_size)?;
        self.process_keypress(win_size)
    }
}

struct Output {
    editor_contents: EditorContents,
    cursor_controller: CursorController,
    editor_rows: EditorRows,
}

impl Output {

    fn new() -> Self {

        Self {
            editor_contents: EditorContents::new(),
            cursor_controller: CursorController::new(),
            editor_rows: EditorRows::new(),
        }
    }

    fn clear_screen() -> Result<(), io::Error>{
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

    fn refresh_screen(&mut self, win_size: (usize, usize)) -> Result<(), io::Error> {
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

    fn move_cursor(&mut self, direction:char, win_size: (usize, usize)) {
        self.cursor_controller.move_cursor(direction, self.editor_rows.number_of_rows(), win_size);
    }
}

struct EditorContents {
    content: String,
}

impl EditorContents {
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    fn push(&mut self, ch: char) {
        self.content.push(ch)
    }

    fn push_str(&mut self, string: &str) {
        self.content.push_str(string)
    }
}

impl io::Write for EditorContents {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.content.push_str(s);
                Ok(s.len())
            }
            Err(_) => Err(io::ErrorKind::WriteZero.into()),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }
}

struct CursorController {
    cursor_x: usize,
    cursor_y: usize,
    row_offset: usize,
}
impl CursorController {
    fn new() -> CursorController {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            row_offset: 0,
        }
    }

    fn move_cursor(&mut self, direction: char, number_of_rows: usize, win_size: (usize, usize)) {
        let screen_columns = win_size.0;
        let screen_rows = win_size.1;
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
    }
}

struct EditorRows {
    row_contents: Vec<Box<str>>,
}

impl EditorRows {
    fn new() -> Self {
        let mut arg = env::args();

        match arg.nth(1) {
            None => Self {
                row_contents: Vec::new(),
            },
            Some(file) => Self::from_file(file.as_ref()),
        }
    }

    fn from_file(file: &Path) -> Self {
        let file_contents = fs::read_to_string(file).expect("Unable to read file");
        Self {
            row_contents: file_contents.lines().map(|it| it.into()).collect(),
        }
    }

    fn number_of_rows(&self) -> usize {
        self.row_contents.len()
    }

    fn get_row(&self, at:usize) -> &str {
        &self.row_contents[at]
    }
}
fn main() -> Result<(), io::Error> {
    let _clean_up = CleanUp;

    terminal::enable_raw_mode()?;

    let mut editor = Editor::new();
    while editor.run()? { }
    Ok(())
}