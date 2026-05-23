use crate::interpreter::{Interpreter, Status};

use ratatui_textarea::{CursorMove, TextArea};

#[derive(Debug)]
pub struct App<'a> {
    pub input_source: TextArea<'a>,
    pub interpreter: Interpreter,
    pub mode: Mode,
    pub is_running: bool,
    pub is_finishing: bool,
}

impl App<'_> {
    pub fn default() -> Self {
        Self {
            input_source: TextArea::default(),
            interpreter: Interpreter::default(),
            mode: Mode::Input,
            is_running: false,
            is_finishing: false,
        }
    }

    pub fn reset(&mut self) {
        let n = App::default();
        *self = n;
    }

    pub fn is_input_mode(&self) -> bool {
        self.mode == Mode::Input
    }

    pub fn is_control_mode(&self) -> bool {
        self.mode == Mode::Control
    }

    pub fn switch_mode(&mut self) {
        match self.mode {
            Mode::Input => {
                self.mode = Mode::Control;
                self.input_source.move_cursor(CursorMove::Jump(0, 0));
            }
            Mode::Control => self.mode = Mode::Input,
        };
    }

    pub fn handle_timeout(&mut self) -> std::io::Result<()> {
        if self.is_finishing {
            while self.interpreter.status != Status::Done {
                self.step()?;
                let p = self.interpreter.current_position();
                self.input_source.move_cursor(CursorMove::Jump(p.0, p.1));
            }

            self.is_finishing = false;

            return Ok(());
        }

        if self.is_running {
            self.step()?;
        }

        Ok(())
    }

    pub fn handle_input(&mut self, key: crossterm::event::KeyEvent) {
        self.input_source.input(key);

        if self.interpreter.status != Status::New {
            self.interpreter.reset();
        }
    }

    pub fn handle_copy_paste(&mut self, s: String) {
        self.input_source.set_yank_text(s);
        self.input_source.paste();
    }

    pub fn step(&mut self) -> std::io::Result<()> {
        match self.interpreter.status {
            Status::Done => {}
            Status::InProgress => {
                self.interpreter.step()?;
                let p = self.interpreter.current_position();
                self.input_source.move_cursor(CursorMove::Jump(p.0, p.1));
            }
            Status::New => {
                let s = self.read_source();
                self.interpreter.tokenize(&s);
                self.interpreter.step()?;
                let p = self.interpreter.current_position();
                self.input_source.move_cursor(CursorMove::Jump(p.0, p.1));
            }
        };

        if self.interpreter.status == Status::Done {
            self.is_running = false;
        }

        Ok(())
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        if self.is_running {
            return Ok(());
        }

        self.interpreter.reset();
        self.is_running = true;

        Ok(())
    }

    pub fn toggle(&mut self) {
        if self.interpreter.status == Status::InProgress {
            self.is_running = !self.is_running;
        }
    }

    pub fn finish(&mut self) {
        self.is_finishing = true;
    }

    fn read_source(&self) -> String {
        let mut s = String::new();
        for line in self.input_source.lines() {
            s.push_str(line);
            s.push_str(";")
        }

        s
    }
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Input,
    Control,
}
