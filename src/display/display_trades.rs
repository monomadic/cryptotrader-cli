use crate::display;
use colored::*;
use cryptotrader;
use cryptotrader::models::*;

pub fn table(trade_groups: Vec<Vec<Trade>>) -> String {
    trade_groups
        .into_iter()
        .map(|trades| {
            format!(
                "{:12}{:9}{:16}{:16}{:16}{:16}\n{}",
                "TYPE",
                "PAIR",
                "QTY",
                "COST",
                "ENTRY",
                "EXIT",
                &trades
                    .into_iter()
                    .map(|trade| table_row(trade))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        })
        .collect::<Vec<String>>()
        .join("\n\n")
}

fn table_row(trade: Trade) -> String {
    format!(
        "{symbol:12}{trade_type:<9}{qty:<16}{cost:<16}{entry_price:<16}{exit_price:<16}",
        symbol = format!("{}-{}", trade.pair.base, trade.pair.symbol).yellow(),
        qty = trade.qty,
        cost = display::pairs::pretty_cost(&trade.pair, trade.cost()),
        trade_type = display::trade_type::colored(trade.trade_type),
        entry_price = display::pairs::pretty_price(&trade.pair),
        exit_price = display::pairs::pretty_price(&trade.pair),
    )
}
