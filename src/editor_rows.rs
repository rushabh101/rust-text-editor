use std::{env, fs};
use std::path::Path;

#[derive(Clone)]
pub struct EditorRows {
    row_contents: Vec<Box<str>>,
}

impl EditorRows {
    pub(crate) fn new() -> Self {
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

    pub(crate) fn number_of_rows(&self) -> usize {
        self.row_contents.len()
    }

    pub(crate) fn get_row(&self, at:usize) -> &str {
        &self.row_contents[at]
    }
}
