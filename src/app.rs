use crate::interpreter::Interpreter;

use ratatui_textarea::TextArea;

#[derive(Debug)]
pub struct App<'a> {
    pub input_source: TextArea<'a>,
    pub interpreter: Interpreter,
}

impl App<'_> {
    pub fn default() -> Self {
        Self {
            input_source: TextArea::default(),
            interpreter: Interpreter::default(),
        }
    }
}
