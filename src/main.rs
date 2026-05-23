mod app;
mod interpreter;
mod view;

use crossterm::event::{self, EnableBracketedPaste, Event, KeyCode};
use crossterm::execute;

use std::io;
use std::time::Duration;

use crate::app::{App, Mode};
use crate::view::View;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App::default();
    let result = run_app(&mut terminal, &mut app);

    ratatui::restore();

    result
}

fn run_app(terminal: &mut ratatui::DefaultTerminal, app: &mut App) -> io::Result<()> {
    execute!(std::io::stdout(), EnableBracketedPaste)?;

    let view = View::default(terminal.get_frame().area());

    loop {
        terminal.draw(|f| view.render(f, app))?;

        let main_event = if event::poll(Duration::from_millis(100))? {
            let event = event::read()?;
            MainEvent::Input(event)
        } else {
            MainEvent::Timeout
        };

        match main_event {
            MainEvent::Timeout => app.handle_timeout()?,
            MainEvent::Input(event) => match app.mode {
                Mode::Input => match event {
                    Event::Paste(s) => app.handle_copy_paste(s),
                    Event::Key(key) => match key.code {
                        KeyCode::Tab => app.switch_mode(),
                        _ => app.handle_input(key),
                    },
                    _ => {}
                },
                Mode::Control => match event {
                    Event::Key(key) => match key.code {
                        KeyCode::Tab => app.switch_mode(),
                        KeyCode::Right => app.step()?,
                        KeyCode::Enter => app.run()?,
                        KeyCode::Esc => app.reset(),
                        KeyCode::Char(' ') => app.toggle(),
                        KeyCode::Char('e') => app.finish(),
                        KeyCode::Char('q') => break,
                        _ => {}
                    },
                    _ => {}
                },
            },
        }
    }

    Ok(())
}

enum MainEvent {
    Input(Event),
    Timeout,
}
