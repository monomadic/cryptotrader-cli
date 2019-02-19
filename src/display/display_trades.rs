use super::*;
use crate::display;
// use colored::*;
use cryptotrader;
// use cryptotrader::models::*;
use cryptotrader::presenters::TradePresenter;

pub fn ticker(presenters: Vec<Vec<TradePresenter>>) -> String {
    presenters
        .into_iter()
        .map(|presenters| {
            if let Some(presenter) = presenters.last() {
                ticker_entry(presenter.clone())
            } else {
                "No trades found.".to_string()
            }
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

pub fn table(presenters: Vec<Vec<TradePresenter>>) -> String {
    let table_content = presenters
        .into_iter()
        .map(|presenters| {
            presenters
                .into_iter()
                .map(|trade_presenter| table_row(trade_presenter.clone())) // FIX THIS
                .collect::<Vec<String>>()
                .join("\n")})
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        "{:normal_width$}{:small_width$}{:normal_width$}{:normal_width$}{:normal_width$}{:normal_width$}{:normal_width$}{:wide_width$}{:normal_width$}\n{}",
        "PAIR",
        "TYPE",
        "ENTRY_PRICE",
        "CURRENT_PRICE",
        "SIZE",
        "QTY",
        "FEE",
        "PROFIT/LOSS",
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
        "{symbol:normal_width$}{trade_type:<small_width$}{entry_price:normal_width$}{current_price:<normal_width$}{size:<normal_width$}{qty: <normal_width$}{fee:normal_width$}{profit:wide_width$}{time:<normal_width$}",
        symbol = format!("{}-{}", trade.pair.symbol, trade.pair.base).yellow(),
        trade_type = display::trade_type::colored(trade.trade_type),
        entry_price = display::pairs::pretty_price_from_base(&trade.pair.base, trade.price),
        size = size(presenter.clone()),
        qty = trade.qty,
        current_price = display::pairs::pretty_price_from_base(&trade.pair.base, trade.pair.price),
        fee = format!(
            "{:.3} {}",
            &trade.fee,
            &trade.clone().fee_symbol.unwrap_or("".to_string())
        ),
        profit = positive_negative(presenter.trade.profit_as_percent(), display_profit(presenter)),
        time = trade.time.format("%Y-%m-%d %H:%M").to_string(),
        small_width = SMALL_COLUMN_WIDTH,
        normal_width = NORMAL_COLUMN_WIDTH,
        wide_width = WIDE_COLUMN_WIDTH,
    )
}

fn display_profit(trade_presenter: TradePresenter) -> String {
    let profit_in_fiat: String = {
        if let Some(profit) = trade_presenter.current_profit_in_fiat() {
            format!(" (${:.2})", profit)
        } else {
            "".to_string()
        }
    };
    let trade = trade_presenter.trade;

    if trade.pair.base_is_fiat() {
        format!(
            "{profit:.2} ({profit_as_percent})",
            profit = print_fiat(trade.profit()),
            profit_as_percent = print_percent(trade.current_profit_as_percent()),
        )
    } else {
        format!(
            "{profit_as_percent}{profit_in_fiat}",
            profit_in_fiat = profit_in_fiat,
            profit_as_percent = print_percent(trade.current_profit_as_percent()),
        )
    }
}

fn size(presenter: TradePresenter) -> String {
    format!(
        "{:.2} (${:.0})",
        presenter.trade.value(),
        presenter.current_cost_in_fiat().expect("trade presenter thing"),
    )
}
