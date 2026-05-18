use crate::app::App;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Style, Text};
use ratatui::widgets::{Block, Cell, Row, Table};

pub struct View {
    layout: Layout,
}

impl View {
    pub fn default() -> Self {
        let layout = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);

        Self { layout }
    }

    pub fn render(&mut self, frame: &mut Frame, app: &mut App) {
        let [left, right] = self.layout.areas(frame.area());

        let block = Block::bordered().title("Source Code");
        app.input_source.set_block(block);
        frame.render_widget(&app.input_source, left);
        let b =
            ratatui::widgets::Paragraph::new(str::from_utf8(&app.interpreter.output()).unwrap());
        frame.render_widget(b, left);

        let mut rows = Vec::new();
        let col_constraints = (0..8).map(|_| Constraint::Length(6));
        for i in 0..16 {
            let mut row = Vec::new();
            for j in 0..8 {
                let id = 8 * i + j;
                let style = if id == app.interpreter.current_cell() {
                    Style::new().red().on_white()
                } else {
                    Style::default()
                };
                let hex = format!("0x{:x}", app.interpreter.get_cell_value(id));
                let text = Text::raw(hex).centered();
                row.push(Cell::from(text).style(style));
            }
            rows.push(Row::new(row));
        }
        let block = Block::bordered().title("Memory Tape");
        let table = Table::new(rows, col_constraints).block(block);
        frame.render_widget(table, right);
    }
}
