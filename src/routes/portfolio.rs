use axum::{Router};
use axum::response::IntoResponse;
use crate::services::solana;

pub fn routes() -> Router {
    Router::new().route("/portfolio/:wallet", get(get_portfolio))
}

async fn get_portfolio(Path(wallet): Path<String>) -> impl IntoResponse {
    match solana::get_solana_balance(&wallet).await {
        Ok(balance_sol) => {
            match coingecko::get_token_price("solana").await {

            }
        }
    }
}