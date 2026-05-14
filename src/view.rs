use crate::app::App;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::{Block, Borders};

pub struct View {
    layout: Layout,
}

impl View {
    pub fn default() -> Self {
        let layout = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);

        Self { layout }
    }

    pub fn render(&mut self, frame: &mut Frame, app: &mut App) {
        let [left, _right] = self.layout.areas(frame.area());

        let block = Block::default().borders(Borders::ALL).title("Source Code");
        app.textarea.set_block(block);

        frame.render_widget(&app.textarea, left);
    }
}
