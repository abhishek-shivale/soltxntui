# SolTxn

A terminal app for looking up Solana transactions, like a tiny Solscan in your terminal.

Paste a transaction signature and SolTxn shows the transaction details: status, slot, block time, fee, compute units, signers, accounts, instructions, and program logs.

Built with Rust and [ratatui](https://ratatui.rs), using the [Helius](https://www.helius.dev) RPC.

## Requirements

- Rust (edition 2024, so Rust 1.85 or newer)
- A Helius API key. You can get one for free at [dashboard.helius.dev](https://dashboard.helius.dev).

## Setup

```sh
git clone https://github.com/abhishek-shivale/soltxntui.git
cd soltxntui
export HELIUS_API_KEY=your-api-key
cargo run
```

SolTxn queries Solana **mainnet**.

## Usage

1. Press `e` to open the input box.
2. Paste a transaction signature (about 88 characters) and press `Enter`.
3. Wait for the loader to finish. The transaction details then appear in a table.
4. Use `↑` / `↓` to scroll through the details.
5. Press `q` or `Esc` to quit.

A transaction signature is the transaction ID. You can copy one from any explorer, such as [Solscan](https://solscan.io). Wallet and token addresses (about 44 characters) are not supported yet.

## Keys

| Key       | Action                       |
|-----------|------------------------------|
| `e`       | Open the input box           |
| `Enter`   | Look up the transaction      |
| `←` / `→` | Move the cursor in the input |
| `↑` / `↓` | Scroll the details           |
| `q` / `Esc` | Quit                       |

## Project layout

```
src/
├── main.rs     App state, input handling, and the render loop
├── rpc.rs      Helius JSON-RPC client (getTransaction)
├── viewer.rs   Transaction details table
└── loading.rs  Loading animation and progress bar
```
