use crate::rpc::SignatureInfo;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Flex, Layout, Position, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Gauge, List, ListState, Paragraph, Row, Table, TableState},
};
use reqwest;
use serde_json::Value;

#[derive(Default)]
pub struct View {
    pub data: Vec<SignatureInfo>,
    pub show: bool,
    pub reocrd: Vec<Table<'static>>
}

impl View {
    pub fn set_data(&mut self, data: Vec<SignatureInfo>) {
        self.data = data;
    }

    pub fn render_data(&self, frame: &mut Frame) {
        if !self.show {
            return;
        }

        for record in self.data.clone() {
            self.render_signature(&record);
        };

    }

    fn render_signature(&self, data: &SignatureInfo) -> Table<'_>  {
        let header = Row::new(["Property", "Value"]).style(Style::default().bold().fg(Color::Blue));

        let json_value = serde_json::to_value(&data).unwrap();

        let mut body: Vec<Row> = vec![];
        if let Value::Object(map) = json_value {
            for (key, vaule) in map {
                body.push(
                    Row::new([format!("{}, {}", key, vaule)])
                        .style(Style::default().fg(Color::Magenta)),
                )
            }
        }

        let widths = [Constraint::Length(20), Constraint::Min(10)];

        Table::new(body, widths)
            .header(header)
            .column_spacing(1)
            .style(Color::White)
            .row_highlight_style(Style::new().on_black().bold())
            .column_highlight_style(Color::Gray)
            .cell_highlight_style(Style::new().reversed().yellow())
    }
}
