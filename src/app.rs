use crate::interpreter::Interpreter;

use ratatui_textarea::TextArea;

#[derive(Debug)]
pub struct App<'a> {
    pub textarea: TextArea<'a>,
    pub interpreter: Interpreter,
}

impl App<'_> {
    pub fn default() -> Self {
        Self {
            textarea: TextArea::default(),
            interpreter: Interpreter::default(),
        }
    }
}
