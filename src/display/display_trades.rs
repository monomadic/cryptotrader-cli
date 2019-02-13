use super::*;
use crate::display;
// use colored::*;
use cryptotrader;
use cryptotrader::models::*;

pub fn table(trade_groups: Vec<Vec<Trade>>) -> String {
    trade_groups
        .into_iter()
        .map(|trades| {
            format!(
                "{:12}{:9}{:16}{:16}{:16}{:16}{:32}{:16}\n{}",
                "PAIR",
                "TYPE",
                "PRICE",
                "QTY",
                "COST",
                "FEE",
                "VALUE",
                "TIME",
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
        "{symbol:12}{trade_type:<9}{entry_price:<16}{qty:<16}{cost:<16}{fee:16}{value:<32}{time:<16}",
        symbol = format!("{}-{}", trade.pair.symbol, trade.pair.base).yellow(),
        qty = trade.qty,
        trade_type = display::trade_type::colored(trade.trade_type),
        entry_price = display::pairs::pretty_price_from_base(&trade.pair.base, trade.price),
        cost = display::pairs::pretty_price_from_base(&trade.pair.base, trade.cost()),
        fee = format!("{} {}", &trade.fee, &trade.clone().fee_symbol.unwrap_or("".to_string())),
        value = display_value_vs_cost(trade.clone()),
        time = trade.time.format("%Y-%m-%d %H:%M:%S").to_string(),
    )
}

fn display_value_vs_cost(trade: Trade) -> String {
    if trade.pair.base_is_fiat() {
        format!(
            "{value} ({profit_as_percent}, {profit})",
            value = print_fiat(trade.value()),
            profit = print_fiat(trade.profit()),
            profit_as_percent = print_percent(trade.profit_as_percent()),
        )
    } else {
        format!(
            "{value} ({profit_as_percent})",
            value = print_btc(trade.value()),
            profit_as_percent = print_percent(trade.profit_as_percent()),
        )
    }
}
