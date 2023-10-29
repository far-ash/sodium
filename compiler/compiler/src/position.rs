use std::fmt::Debug;

#[derive(Clone)]
pub struct Position {
    file: String,
    line: u16,
    column: u8,
}

pub enum Next {
    Line,
    Column,
}

impl Position {
    pub fn new(file: &str) -> Self {
        Self {
            file: file.to_owned(),
            line: 1,
            column: 1,
        }
    }

    pub fn file(&self) -> String {
        self.file.clone()
    }

    pub const fn line(&self) -> u16 {
        self.line
    }

    pub const fn column(&self) -> u8 {
        self.column
    }

    pub fn next(&mut self, by: Next) {
        match by {
            Next::Line => {
                self.line += 1;
                self.column = 1
            }
            Next::Column => self.column += 1,
        }
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{file}:{line}:{column}",
            file = self.file,
            line = self.line,
            column = self.column
        )
    }
}
