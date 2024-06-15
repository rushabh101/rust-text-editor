use crossterm::{event, terminal};
use std::io::{Read, Write};
use std::time::Duration;
use crossterm::event::{Event, KeyEvent};
use std::io;
use editor::Editor;
use output::Output;

mod editor;
mod output;
mod editor_contents;
mod cursor_controller;
mod editor_rows;

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

fn main() -> Result<(), io::Error> {
    let _clean_up = CleanUp;

    terminal::enable_raw_mode()?;

    let mut editor = Editor::new();
    while editor.run()? { }
    Ok(())
}
