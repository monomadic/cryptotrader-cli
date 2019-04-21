use crate::display::*;
use crate::utils::*;
use cryptotrader;
use cryptotrader::models::*;
use cryptotrader::presenters::*;

pub fn ticker(exchange: &str, presenter: BalancePresenter) -> String {
    presenter
        .assets
        .iter()
        .map(|asset| format!("{:16}{:16.2}", asset.symbol, asset.amount))
        .collect::<Vec<String>>()
        .join("\n")
}
