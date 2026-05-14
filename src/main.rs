mod app;
mod interpreter;
mod view;

use crossterm::event::{self, EnableBracketedPaste, Event, KeyCode};
use crossterm::execute;

use std::io;

use crate::app::App;
use crate::view::View;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App::default();
    let mut view = View::default();

    let result = run_app(&mut terminal, &mut app, &mut view);

    ratatui::restore();

    result
}

fn run_app(
    terminal: &mut ratatui::DefaultTerminal,
    app: &mut App,
    view: &mut View,
) -> io::Result<()> {
    // let mut interpreter = Interpreter::default();

    // interpreter.tokenize(&app.source);
    // interpreter.run()?;

    execute!(std::io::stdout(), EnableBracketedPaste)?;

    loop {
        terminal.draw(|f| view.render(f, app))?;

        match event::read()? {
            Event::Paste(s) => {
                app.textarea.set_yank_text(s);
                app.textarea.paste();
            }
            Event::Key(key) => match key.code {
                KeyCode::Esc => break,
                _ => {
                    app.textarea.input(key);
                }
            },
            _ => {}
        };

        // app.update(message);
    }

    Ok(())
}
