use std::io;
use crossterm::event::{KeyCode, KeyEvent};
use crossterm::{event, terminal};
use crate::Reader;
use crate::output::Output;

pub struct Editor {
    reader: Reader,
    output: Output,
}

impl Editor {
    pub(crate) fn new() -> Self {
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

    pub(crate) fn run(&mut self) -> Result<bool, io::Error> {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();

        self.output.refresh_screen(win_size)?;
        self.process_keypress(win_size)
    }
}
