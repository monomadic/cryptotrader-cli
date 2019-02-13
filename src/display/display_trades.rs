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
                "{:normal_width$}{:small_width$}{:normal_width$}{:normal_width$}{:normal_width$}{:normal_width$}{:normal_width$}{:normal_width$}{:normal_width$}\n{}",
                "PAIR",
                "TYPE",
                "ENTRY_PRICE",
                "CURRENT_PRICE",
                "SIZE",
                "QTY",
                "FEE",
                "PROFIT/LOSS",
                "TIME",
                &trades
                    .into_iter()
                    .map(|trade| table_row(trade))
                    .collect::<Vec<String>>()
                    .join("\n"),
                small_width = SMALL_COLUMN_WIDTH,
                normal_width = NORMAL_COLUMN_WIDTH,
            )
        })
        .collect::<Vec<String>>()
        .join("\n\n")
}

fn table_row(trade: Trade) -> String {
    format!(
        "{symbol:normal_width$}
        {trade_type:<small_width$}
        {entry_price:normal_width$}
        {current_price:<normal_width$}
        {size:<normal_width$}
        {qty: <normal_width$}
        {fee:normal_width$}
        {profit:normal_width$}
        {time:<normal_width$}",
        symbol = format!("{}-{}", trade.pair.symbol, trade.pair.base).yellow(),
        trade_type = display::trade_type::colored(trade.trade_type),
        entry_price = display::pairs::pretty_price_from_base(&trade.pair.base, trade.price),
        size = format!("{:.3}", trade.value()),
        qty = trade.qty,
        current_price = display::pairs::pretty_price_from_base(&trade.pair.base, trade.pair.price),
        fee = format!(
            "{:.3} {}",
            &trade.fee,
            &trade.clone().fee_symbol.unwrap_or("".to_string())
        ),
        profit = positive_negative(trade.profit(), display_profit(trade.clone())),
        time = trade.time.format("%Y-%m-%d %H:%M").to_string(),
        small_width = SMALL_COLUMN_WIDTH,
        normal_width = NORMAL_COLUMN_WIDTH,
    )
}

fn display_profit(trade: Trade) -> String {
    if trade.pair.base_is_fiat() {
        format!(
            "{profit:.2} ({profit_as_percent})",
            profit = print_fiat(trade.profit()),
            profit_as_percent = print_percent(trade.profit_as_percent()),
        )
    } else {
        format!(
            "{profit_as_percent}",
            profit_as_percent = print_percent(trade.profit_as_percent()),
        )
    }
}
