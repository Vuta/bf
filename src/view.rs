use crate::app::App;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Rect, Style, Text};
use ratatui::widgets::{Block, Cell, Paragraph, Row, Table};

pub struct View {
    source_panel: Rect,
    memory_panel: Rect,
    output_panel: Rect,
}

impl View {
    pub fn default(area: Rect) -> Self {
        let layout = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [left, right] = layout.areas(area);
        let [memory, output] = Layout::vertical([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]).areas(right);

        Self {
            source_panel: left,
            memory_panel: memory,
            output_panel: output,
        }
    }

    pub fn render(&self, frame: &mut Frame, app: &mut App) {
        // render source code
        let block = Block::bordered().title("Source Code");
        app.input_source.set_block(block);
        frame.render_widget(&app.input_source, self.source_panel);

        // render memory tape
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
        frame.render_widget(table, self.memory_panel);

        // render output
        let block = Block::bordered().title("Output");
        let b = Paragraph::new(str::from_utf8(&app.interpreter.output()).unwrap()).block(block);
        frame.render_widget(b, self.output_panel);
    }
}
