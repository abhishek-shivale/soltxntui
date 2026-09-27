use crate::rpc::SignatureInfo;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Flex, Layout, Position, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Gauge, List, ListState, Paragraph},
};
use reqwest;

#[derive(Default)]
pub struct View {
    pub data: Vec<SignatureInfo>,
    pub show: bool
}

impl View {
    pub fn new(&mut self, data: Vec<SignatureInfo>){
        self.data = data;
    }

    pub fn render_data(&self, frame: &mut Frame) {
        if !self.show { return }
        frame.render_widget(Paragraph::new("ViewRendered"), frame.area());
    }

    fn render_signature() {}

//     pub fn render_err(&self, frame: &mut Frame, erro: Err) {
//         if let Some(Err(err)) = &self.outcome {
//     frame.render_widget(Paragraph::new(format!("{err:?}")), frame.area());
// }uuu
//     }
}
