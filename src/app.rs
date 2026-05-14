use std::fs::File;
use std::io::{BufReader, Read};

use crate::interpreter::Interpreter;

use ratatui_textarea::TextArea;

#[derive(Debug)]
pub struct App<'a> {
    // TODO: remove path once source input is implemented
    pub path: String,
    pub source: String,
    pub textarea: TextArea<'a>,
    pub interpreter: Interpreter,
}

impl App<'_> {
    pub fn default() -> Self {
        Self {
            path: String::new(),
            source: String::new(),
            textarea: TextArea::default(),
            interpreter: Interpreter::default(),
        }
    }

    pub fn store_path(&mut self, path: String) {
        self.path = path;
    }

    pub fn read_source(&mut self) -> std::io::Result<()> {
        let file = File::open(&self.path)?;
        let mut buf_reader = BufReader::new(file);

        buf_reader.read_to_string(&mut self.source)?;

        self.textarea = TextArea::from(self.source.lines());

        Ok(())
    }
}
