#![allow(unused_variables)]

use std::time::Duration;

use color_eyre::{
    eyre::{Result, WrapErr},
    install,
};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Flex, Layout, Position, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Gauge, List, ListState, Paragraph},
};
mod loading;

#[derive(Default)]
struct App {
    id: Vec<String>,
    input_mode: bool,
    exit: bool,
    character_index: usize,
    input: String,
    search_mode: bool,
    loading: loading::Loading,
}

fn main() -> Result<()> {
    install()?; // color eyre added here 
    ratatui::run(|terminal| App::default().run(terminal))?;
    Ok(())
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|f| self.render(f))?;
            if event::poll(Duration::from_millis(50))? {
                self.handle_events()?;
            }
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let prompt = Line::from("SolTxn".to_string()).style(Style::default().fg(Color::Blue));
        frame.render_widget(Paragraph::new(prompt), frame.area());
        self.render_dialogue_txn_id(frame);
        self.loading.draw(frame);
    }

    fn render_dialogue_txn_id(&mut self, frame: &mut Frame) -> Result<()> {
        if self.input_mode {
            let area = self.pop_up(frame.area(), 50, 3);
            let input = Paragraph::new(self.input.as_str())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::bordered().title("Enter token, validator, programs and accounts"));

            frame.render_widget(input, area);

            frame.set_cursor_position(Position::new(
                area.x + 1 + self.input.chars().count() as u16,
                area.y + 1,
            ));
        }

        Ok(())
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        if self.input_mode {
            let index = self.byte_index();
            self.input.insert(index, new_char);
            self.move_cursor_right();
        } else {
            if new_char == 'e' {
                self.input_mode = true
            }
        }
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    const fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn submit_message(&mut self) {
        self.id.push(self.input.clone());
        self.input.clear();
        self.reset_cursor();
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn pop_up(&self, area: Rect, percent_x: u16, height: u16) -> Rect {
        let [area] = Layout::vertical([Constraint::Length(height)])
            .flex(Flex::Center)
            .areas(area);
        let [area] = Layout::horizontal([Constraint::Percentage(percent_x)])
            .flex(Flex::Center)
            .areas(area);
        area
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(k) = event::read()? {
            if k.kind == KeyEventKind::Press {
                self.handle_key(k);
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.exit = true,
            KeyCode::Char(to_insert) => self.enter_char(to_insert),
            KeyCode::Backspace => self.delete_char(),
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Right => self.move_cursor_right(),
            KeyCode::Enter => self.handle_enter(),
            // KeyCode::Char('e') => self.input_mode = true,
            // KeyCode::Esc => self.input_mode = false,
            _ => {}
        }
    }

    fn handle_enter(&mut self) {
        if self.input_mode {
            self.input_mode = false;
            self.loading.state = true;
            self.search_mode = true;
            // self.submit_message();
            // self.loading.start("Searching");
        }
    }

    fn handle_loading(&mut self, frame: &mut Frame) {}
}
