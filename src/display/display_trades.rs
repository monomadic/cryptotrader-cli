use super::*;
use crate::display;
use crate::display::display_pairs::display_pair;
// use colored::*;
use cryptotrader;
use cryptotrader::models::*;
use cryptotrader::presenters::TradePresenter;
use prettytable::{cell, row, Row};

pub fn _table_row(presenter: TradePresenter) -> Row {
    let trade = presenter.trade.clone();
    let indicator = match trade.trade_type {
        TradeType::Buy => "+".green(),
        TradeType::Sell => "-".red(),
    };

    row!(
        format!("{} {}", indicator, display_pair(&trade.pair).yellow()),
        display::pairs::pretty_price_from_base(&trade.pair.base, trade.price),
        size(presenter.clone()),
        display::pairs::pretty_price_from_base(&trade.pair.base, trade.price),
        format!("{:.2}", presenter.trade.qty),
    )
}

pub fn ticker(presenters: Vec<TradePresenter>) -> String {
    presenters
        .into_iter()
        .map(|presenter| {
            ticker_entry(presenter.clone())
            // if let Some(presenter) = presenters.last() {
            //     ticker_entry(presenter.clone())
            // } else {
            //     "No trades found.".to_string()
            // }
        })
        .collect::<Vec<String>>()
        .join(" :: ")
}

fn ticker_entry(presenter: TradePresenter) -> String {
    format!(
        "{symbol} {size} {profit}",
        symbol = format!(
            "{}-{}",
            presenter.trade.pair.symbol, presenter.trade.pair.base
        )
        .yellow(),
        size = size(presenter.clone()),
        profit = positive_negative(
            presenter.trade.profit_as_percent(),
            display_profit(presenter)
        ),
    )
}

pub fn table(presenters: Vec<TradePresenter>) -> String {
    let table_content = presenters
        .into_iter()
        .map(|presenter| {
            table_row(presenter.clone())
            // presenters
            //     .into_iter()
            //     .map(|trade_presenter| table_row(trade_presenter.clone())) // FIX THIS
            //     .collect::<Vec<String>>()
            //     .join("\n")
        })
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        "{:normal_width$}{:small_width$}{:normal_width$}{:normal_width$}{:normal_width$}{:normal_width$}{:normal_width$}{:wide_width$}{:normal_width$}\n{}",
        "PAIR",
        "TYPE",
        "PRICE",
        "SIZE",
        "QTY",
        "FEE",
        "CURRENT_PRICE",
        "DISTANCE",
        "TIME",
        table_content,
            small_width = SMALL_COLUMN_WIDTH,
            normal_width = NORMAL_COLUMN_WIDTH,
            wide_width = WIDE_COLUMN_WIDTH,
    )
}

fn table_row(presenter: TradePresenter) -> String {
    let trade = presenter.trade.clone();
    format!(
        "{symbol:normal_width$}{trade_type:<small_width$}{entry_price:normal_width$}{size:<normal_width$}{qty: <normal_width$}{fee:normal_width$}{current_price:<normal_width$}{profit:wide_width$}{time:<normal_width$}",
        symbol = format!("{}-{}", trade.pair.symbol, trade.pair.base).yellow(),
        trade_type = display::trade_type::colored(trade.trade_type),
        entry_price = display::pairs::pretty_price_from_base(&trade.pair.base, trade.price),
        size = size(presenter.clone()),
        qty = display_qty(trade.qty),
        fee = format!(
            "{:.3} {}",
            &trade.fee,
            &trade.clone().fee_symbol.unwrap_or("".to_string())
        ),
        current_price = display::pairs::pretty_price_from_base(&trade.pair.base, trade.pair.price),
        profit = positive_negative(presenter.trade.profit_as_percent(), display_profit(presenter)),
        time = trade.time.format("%Y-%m-%d %H:%M").to_string(),
        small_width = SMALL_COLUMN_WIDTH,
        normal_width = NORMAL_COLUMN_WIDTH,
        wide_width = WIDE_COLUMN_WIDTH,
    )
}

fn display_profit(presenter: TradePresenter) -> String {
    let current_profit_as_percent: f64 = presenter.trade.current_profit_as_percent();
    let current_profit_in_fiat: String = match presenter.current_profit_in_fiat() {
        Some(profit) => format!(" ({})", print_fiat(profit)),
        None => "".to_string(),
    };

    format!(
        "{}{}",
        print_percent(current_profit_as_percent),
        current_profit_in_fiat,
    )
}

fn size(presenter: TradePresenter) -> String {
    match presenter.trade.pair.base_type() {
        AssetType::Fiat => format!("${:.2}", presenter.trade.value()),
        AssetType::Bitcoin | AssetType::Altcoin => format!(
            "{:.2} (${:.0})",
            presenter.trade.value(),
            presenter.current_cost_in_fiat().unwrap_or(0.0),
        ),
    }
}

fn display_qty(qty: f64) -> String {
    format!("{:.2}", qty)
}
