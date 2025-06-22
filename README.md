# 🦀 Token Portfolio Tracker (Rust)

A lightweight Rust-powered API service that tracks crypto wallet balances and portfolio value across multiple chains, starting with **Solana**. Real-time price data is fetched from CoinGecko.


## Features
- Query wallet token balances(starting with Solana)
- Fetch real-time USD prices from CoinGecko
- Calculate total portfolio value
- Fast, async Rust backend using Axum and Tokio
- Modular design (easy to extend to Ethereum, SPL, ERC20, etc.)

## Future roadmap

-	Add ERC20 token support via ethers-rs + Alchemy
-	Track SPL token balances on Solana
-	Add frontend with Tauri or React
-	Integrate Redis for caching prices
-	Multi-currency support (USDT, EUR, etc.)
-	Token icon, symbol, 24h price change

## Tech Stack
| Layer       | Tech Used                             |
|-------------|----------------------------------------|
| Language    | [Rust](https://www.rust-lang.org/)     |
| Web Server  | [Axum](https://docs.rs/axum/)          |
| Async I/O   | [Tokio](https://tokio.rs/)             |
| HTTP Client | [Reqwest](https://docs.rs/reqwest/)    |
| JSON        | [Serde](https://serde.rs/)             |
| API Data    | [CoinGecko](https://www.coingecko.com/en/api) + Solana RPC |
---

## Getting Started

### 1. Clone the Repo
```cookie
git clone https://github.com/KevinSheeranxyj/rust-token-tracker.git
```
