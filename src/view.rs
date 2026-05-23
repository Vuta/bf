use crate::app::App;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Rect, Style, Text};
use ratatui::widgets::{Block, BorderType, Cell, Paragraph, Row, Table};

pub struct View {
    source_panel: Rect,
    memory_panel: Rect,
    output_panel: Rect,
    control_panel: Rect,
}

impl View {
    pub fn default(area: Rect) -> Self {
        let [layout, control] =
            Layout::vertical([Constraint::Percentage(90), Constraint::Percentage(10)]).areas(area);

        let [left, right] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(layout);

        let [memory, output] =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(right);

        Self {
            source_panel: left,
            memory_panel: memory,
            output_panel: output,
            control_panel: control,
        }
    }

    pub fn render(&self, frame: &mut Frame, app: &mut App) {
        let block = if app.is_input_mode() {
            Block::bordered()
                .title("Source Code")
                .border_type(BorderType::Thick)
        } else {
            Block::bordered().title("Source Code")
        };
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
        let p = Paragraph::new(str::from_utf8(&app.interpreter.output()).unwrap()).block(block);
        frame.render_widget(p, self.output_panel);

        // render control
        let block = if app.is_control_mode() {
            Block::bordered()
                .title("Control")
                .border_type(BorderType::Thick)
        } else {
            Block::bordered().title("Control")
        };

        let p = Paragraph::new(
            "Arrow→: Step       Enter↵: Run      Space␣: Pause/Resume      Esc: Reset     Tab⇥: Switch mode     q: Quit",
        )
        .block(block)
        .centered();
        frame.render_widget(p, self.control_panel);
    }
}
