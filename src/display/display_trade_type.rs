use colored::*;
use cryptotrader::models::*;

pub fn colored(state: TradeType) -> ColoredString {
    match state {
        TradeType::Buy => "BUY".green(),
        TradeType::Sell => "SELL".red(),
    }
}
