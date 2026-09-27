use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Gauge, Paragraph},
};
use std::time::{Duration, Instant};

pub struct Loading {
    pub state: bool,
    pub message: String,
    pub ratio: f64,
    tick: usize,
    last_tick: Instant,
}

impl Default for Loading {
    fn default() -> Self {
        Self {
            state: false,
            message: "Loading".into(),
            ratio: 0.0,
            tick: 0,
            last_tick: Instant::now(),
        }
    }
}

impl Loading {
    const IDEL: &[&str] = &[
        r"      .-~~~-.      ",
        r"   .-(  o o  )-.   ",
        r"  (      w      )  ",
        r"   '-.._____..-'   ",
        r"                   ",
        r"                   ",
    ];

    const BLINK: &[&str] = &[
        r"      .-~~~-.      ",
        r"   .-(  - -  )-.   ",
        r"  (      w      )  ",
        r"   '-.._____..-'   ",
        r"                   ",
        r"                   ",
    ];

    const ZAP_A: &[&str] = &[
        r"      .-~~~-.      ",
        r"   .-(  > <  )-.   ",
        r"  (      O      )  ",
        r"   '-.._____..-'   ",
        r"      /_   /_      ",
        r"       /    /      ",
    ];

    const ZAP_B: &[&str] = &[
        r"    * .-~~~-. *    ",
        r"   .-(  > <  )-.   ",
        r"  (      O      )  ",
        r"   '-.._____..-'   ",
        r"       _\   _\     ",
        r"        \    \     ",
    ];

    const HAPPY: &[&str] = &[
        r"      .-~~~-.      ",
        r"   .-(  ^ ^  )-.   ",
        r"  (      v      )  ",
        r"   '-.._____..-'   ",
        r"                   ",
        r"                   ",
    ];

    const FRAMES: [&[&str]; 5] = [
        Loading::IDEL,
        Loading::BLINK,
        Loading::ZAP_A,
        Loading::ZAP_B,
        Loading::HAPPY,
    ];
    const SEQUENCE: [usize; 12] = [0, 0, 0, 1, 0, 0, 0, 2, 3, 2, 3, 0];

    const TICK: Duration = Duration::from_millis(200);
    const GAUGE_WIDTH: u16 = 32;

    pub fn draw(&mut self, frame: &mut Frame) {
        if !self.state {
            return;
        }

        if self.last_tick.elapsed() >= Self::TICK {
            self.tick = self.tick.wrapping_add(1);
            self.last_tick = Instant::now();
            self.ratio += (0.95 - self.ratio) * 0.08;
        }

        let art = Self::FRAMES[Self::SEQUENCE[self.tick % Self::SEQUENCE.len()]];
        let text = Self::sprite(art);

        let [sprite_area, _, lable_area, gauage_area] = Layout::vertical([
            Constraint::Length(text.height() as u16),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .flex(Flex::Center)
        .areas(frame.area());

        let w = text.width() as u16;
        frame.render_widget(Paragraph::new(text), Self::centered(sprite_area, w));

        let dots = (self.tick / 2) % 4;

        let lable = format!("{}{}", self.message, ".".repeat(dots),);

        frame.render_widget(
            Paragraph::new(lable),
            Self::centered(lable_area, Self::GAUGE_WIDTH),
        );

        let ratio = self.ratio.clamp(0.0, 1.0);

        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(Color::Yellow).bg(Color::Black))
            .ratio(ratio)
            .label(format!("{:>3}%", (ratio * 100.0).round() as u32));

        frame.render_widget(gauge, Self::centered(gauage_area, Self::GAUGE_WIDTH));
    }

    fn centered(area: Rect, width: u16) -> Rect {
        let [r] = Layout::horizontal([Constraint::Length(width)])
            .flex(Flex::Center)
            .areas(area);
        r
    }

    fn sprite(art: &[&str]) -> Text<'static> {
        let cloud = Style::default().fg(Color::Gray);
        let bolt = Style::default().fg(Color::Yellow);
        let lines: Vec<Line> = art
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let row_style = if i >= 4 { bolt } else { cloud };
                Line::from(
                    row.chars()
                        .map(|c| {
                            Span::styled(c.to_string(), if c == '*' { bolt } else { row_style })
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect();

        Text::from(lines)
    }

    pub fn new(message: impl Into<String>) -> Self {
        Self {
            state: false,
            message: message.into(),
            ratio: 0.0,
            tick: 0,
            last_tick: Instant::now(),
        }
    }

    pub fn start(&mut self, message: impl Into<String>) {
        self.state = true;
        self.message = message.into();
        self.ratio = 0.0;
        self.tick = 0;
        self.last_tick = Instant::now();
    }

    pub fn stop(&mut self) {
        self.state = false;
    }
}
