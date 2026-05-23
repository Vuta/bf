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
    let result = run_app(&mut terminal, &mut app);

    ratatui::restore();

    result
}

fn run_app(
    terminal: &mut ratatui::DefaultTerminal,
    app: &mut App,
) -> io::Result<()> {
    execute!(std::io::stdout(), EnableBracketedPaste)?;

    let view = View::default(terminal.get_frame().area());

    loop {
        terminal.draw(|f| view.render(f, app))?;

        match event::read()? {
            Event::Paste(s) => {
                app.input_source.set_yank_text(s);
                app.input_source.paste();
            }
            Event::Key(key) => match key.code {
                KeyCode::Esc => {
                    app.input_source.clear();
                    app.interpreter.reset();
                }
                KeyCode::Char('q') => break,
                KeyCode::Char('r') => {
                    app.interpreter.reset();

                    let mut s = String::new();
                    for line in app.input_source.lines() {
                        s.push_str(line);
                    }
                    app.interpreter.tokenize(&s);
                    app.interpreter.run()?;
                }
                KeyCode::Char('s') => {
                    if !app.interpreter.is_finish() {
                        let mut s = String::new();
                        for line in app.input_source.lines() {
                            s.push_str(line);
                        }
                        app.interpreter.tokenize(&s);
                        app.interpreter.step()?;
                    }
                }
                _ => {
                    app.input_source.input(key);
                }
            },
            _ => {}
        };
    }

    Ok(())
}
