use crate::rpc::SignatureInfo;
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Row, Table, TableState},
};

#[derive(Default)]
pub struct View {
    pub data: Vec<SignatureInfo>,
    pub show: bool,
    pub table_state: TableState,
}

impl View {
    pub fn set_data(&mut self, data: Vec<SignatureInfo>) {
        self.data = data;
        self.table_state.select(if self.data.is_empty() { None } else { Some(0) });
    }

    pub fn next_row(&mut self) {
        self.table_state.select_next();
    }

    pub fn previous_row(&mut self) {
        self.table_state.select_previous();
    }

    pub fn render_data(&mut self, frame: &mut Frame) {
        if !self.show {
            return;
        }
        let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
        let [top, main] = frame.area().layout(&layout);

        let title = Line::from_iter([
            Span::from("Transactions").bold(),
            Span::from(format!(" ({}) ", self.data.len())),
            Span::from("(Press 'q' to quit and ↑/↓ to navigate)"),
        ]);
        frame.render_widget(title.centered(), top);

        if self.data.is_empty() {
            frame.render_widget(Line::from("No transactions").centered(), main);
            return;
        }

        let table = self.render_table();
        frame.render_stateful_widget(table, main, &mut self.table_state);
    }

    fn render_table(&self) -> Table<'static> {
        let header = Row::new(["Signature", "Slot", "Block Time", "Status", "Result"])
            .style(Style::default().bold().fg(Color::Blue))
            .bottom_margin(1);

        let body: Vec<Row> = self.data.iter().map(Self::render_row).collect();

        let widths = [
            Constraint::Fill(1),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(10),
            Constraint::Length(7),
        ];

        Table::new(body, widths)
            .header(header)
            .block(Block::bordered())
            .column_spacing(1)
            .style(Color::White)
            .row_highlight_style(Style::new().on_black().bold())
            .highlight_symbol("> ")
    }

    fn render_row(sig: &SignatureInfo) -> Row<'static> {
        let block_time = sig
            .block_time
            .map_or("-".to_string(), |t| t.to_string());

        let result = match sig.err {
            Some(_) => Span::from("Failed").red(),
            None => Span::from("Ok").green(),
        };

        Row::new(vec![
            Line::from(sig.signature.clone()),
            Line::from(sig.slot.to_string()),
            Line::from(block_time),
            Line::from(sig.confirmation_status.clone()),
            Line::from(result),
        ])
    }
}
