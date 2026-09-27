use crate::rpc::TransactionDetail;
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Row, Table, TableState},
};

#[derive(Default)]
pub struct View {
    // pub data: Vec<SignatureInfo>,
    pub data: Option<TransactionDetail>,
    pub show: bool,
    pub table_state: TableState,
}

impl View {
    // pub fn set_data(&mut self, data: Vec<SignatureInfo>) {
    //     self.data = data;
    //     self.table_state.select(if self.data.is_empty() { None } else { Some(0) });
    // }

    pub fn set_data(&mut self, data: TransactionDetail) {
        self.data = Some(data);
        self.table_state.select(Some(0));
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
            Span::from("Transaction Details").bold(),
            Span::from("  ↑/↓ scroll").dark_gray(),
        ]);
        frame.render_widget(title.centered(), top);

        let Some(tx) = &self.data else {
            frame.render_widget(Line::from("No transaction").centered(), main);
            return;
        };

        let table = Self::render_table(tx);
        frame.render_stateful_widget(table, main, &mut self.table_state);
    }

    fn render_table(tx: &TransactionDetail) -> Table<'static> {
        let header = Row::new(["Property", "Value"])
            .style(Style::default().bold().fg(Color::Blue))
            .bottom_margin(1);

        let widths = [Constraint::Length(20), Constraint::Fill(1)];

        Table::new(Self::render_rows(tx), widths)
            .header(header)
            .block(Block::bordered())
            .column_spacing(1)
            .style(Color::White)
            .row_highlight_style(Style::new().on_black().bold())
            .highlight_symbol("> ")
    }

    fn render_rows(tx: &TransactionDetail) -> Vec<Row<'static>> {
        let meta = tx.meta.as_ref();
        let message = &tx.transaction.message;

        let result = match meta.and_then(|m| m.err.as_ref()) {
            Some(err) => Span::from(format!("Failed {err}")).red(),
            None => Span::from("Success").green(),
        };

        let block_time = tx.block_time.map_or("-".to_string(), |t| t.to_string());

        let fee = meta.map_or("-".to_string(), |m| {
            format!("{} SOL", m.fee as f64 / 1_000_000_000.0)
        });

        let compute_units = meta
            .and_then(|m| m.compute_units_consumed)
            .map_or("-".to_string(), |c| c.to_string());

        let version = tx.version.as_ref().map_or("-".to_string(), |v| match v {
            serde_json::Value::String(s) => s.clone(),
            other => other.to_string(),
        });

        let mut rows = vec![
            Self::row("Signature", tx.transaction.signatures.first().cloned().unwrap_or_default()),
            Row::new(vec![Line::from("Result"), Line::from(result)]),
            Self::row("Slot", tx.slot.to_string()),
            Self::row("Block Time", block_time),
            Self::row("Fee", fee),
            Self::row("Compute Units", compute_units),
            Self::row("Version", version),
            Self::row("Recent Blockhash", message.recent_blockhash.clone()),
        ];

        let signers = message.account_keys.iter().filter(|k| k.signer);
        rows.extend(Self::list("Signers", signers.map(|k| k.pubkey.clone())));

        let accounts = message.account_keys.iter().map(|k| {
            let rw = if k.writable { "writable" } else { "readonly" };
            format!("{} ({rw})", k.pubkey)
        });
        rows.extend(Self::list("Accounts", accounts));

        let instructions = message.instructions.iter().enumerate().map(|(i, ix)| {
            let program = ix.program.clone().unwrap_or_else(|| ix.program_id.clone());
            format!("#{} {program}", i + 1)
        });
        rows.extend(Self::list("Instructions", instructions));

        let logs = meta.and_then(|m| m.log_messages.clone()).unwrap_or_default();
        rows.extend(Self::list("Logs", logs.into_iter()));

        rows
    }

    fn row(key: &'static str, value: String) -> Row<'static> {
        Row::new(vec![Line::from(key), Line::from(value)])
    }

    fn list(key: &'static str, values: impl Iterator<Item = String>) -> Vec<Row<'static>> {
        values
            .enumerate()
            .map(|(i, v)| Self::row(if i == 0 { key } else { "" }, v))
            .collect()
    }

    // fn render_table(&self) -> Table<'static> {
    //     let header = Row::new(["Signature", "Slot", "Block Time", "Status", "Result"])
    //         .style(Style::default().bold().fg(Color::Blue))
    //         .bottom_margin(1);
    //
    //     let body: Vec<Row> = self.data.iter().map(Self::render_row).collect();
    //
    //     let widths = [
    //         Constraint::Fill(1),
    //         Constraint::Length(12),
    //         Constraint::Length(12),
    //         Constraint::Length(10),
    //         Constraint::Length(7),
    //     ];
    //
    //     Table::new(body, widths)
    //         .header(header)
    //         .block(Block::bordered())
    //         .column_spacing(1)
    //         .style(Color::White)
    //         .row_highlight_style(Style::new().on_black().bold())
    //         .highlight_symbol("> ")
    // }
    //
    // fn render_row(sig: &SignatureInfo) -> Row<'static> {
    //     let block_time = sig
    //         .block_time
    //         .map_or("-".to_string(), |t| t.to_string());
    //
    //     let result = match sig.err {
    //         Some(_) => Span::from("Failed").red(),
    //         None => Span::from("Ok").green(),
    //     };
    //
    //     Row::new(vec![
    //         Line::from(sig.signature.clone()),
    //         Line::from(sig.slot.to_string()),
    //         Line::from(block_time),
    //         Line::from(sig.confirmation_status.clone()),
    //         Line::from(result),
    //     ])
    // }
}
