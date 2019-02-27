use super::*;
use cryptotrader::models::AssetType;
use cryptotrader::presenters::*;
use prettytable::{cell, row, Row, Table};

pub fn ticker(presenters: Vec<PositionPresenter>) -> String {
    presenters
        .into_iter()
        .map(|presenter| {
            format!(
                "{symbol} {is_valid}{value_btc:.2} btc {profit_percent} {profit_usd}",
                symbol = presenter.symbol().yellow(),
                is_valid = print_bool(presenter.is_valid()),
                value_btc = presenter.current_value_in_btc(),
                profit_percent = positive_negative(
                    presenter.percent_change(),
                    format!("{:.2}%", presenter.percent_change())
                ),
                profit_usd = positive_negative(
                    presenter.unrealised_profit_usd(),
                    format!("(${:.2})", presenter.unrealised_profit_usd())
                ),
            )
        })
        .collect::<Vec<String>>()
        .join(" :: ")
}

pub fn table_row(presenter: &PositionPresenter) -> Row {
    row!(
        presenter.symbol().yellow(),
        display_size(presenter.clone()),
        display_number_by_asset_type(
            presenter.position.entry_price(),
            presenter.position.asset.asset_type()
        ),
        presenter
            .position
            .exit_price()
            .map_or("".to_string(), |exit_price| display_number_by_asset_type(
                exit_price,
                presenter.position.asset.asset_type()
            )),
        display_profit(
            presenter.percent_change(),
            presenter.unrealised_profit_usd()
        )
        .to_string(),
        display_profit(presenter.percent_change(), presenter.realised_profit_usd()),
        presenter
            .position
            .trades
            .first()
            .map(|trade| trade.time.format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or("-".to_string())
    )
}

pub fn table(presenters: Vec<PositionPresenter>) -> String {
    let mut table = Table::new();
    table.set_format(table_format());
    table.set_titles(row!(
        "SYMBOL",
        "SIZE",
        "ENTRY_PRICE",
        "EXIT_PRICE",
        "UPNL",
        "RPNL",
        "TIME"
    ));

    for presenter in presenters {
        table.add_row(table_row(&presenter));
    }

    format!("{}", table)
}

// pub fn _table(presenters: Vec<PositionPresenter>) -> String {
//     format!(
//         "{:normal_width$}{:1}{:wide_width$}{:normal_width$}{:normal_width$}{:wide_width$}{:wide_width$}{:wide_width$}\n{}",
//         "SYMBOL", " ", "SIZE", "ENTRY_PRICE", "EXIT_PRICE", "UPNL", "RPNL","TIME",
//         &presenters.into_iter().map(|presenter| {
//             let position = presenter.position.clone();
//             info!("table({})", position.symbol());

//             format!("{symbol:normal_width$}{valid:1}{size:<wide_width$}{entry_price:<normal_width$.8}{exit_price:<normal_width$}{upnl:wide_width$}{rpnl:<wide_width$}{time:<normal_width$}",
//                 symbol                      = position.symbol().yellow(),
//                 valid                       = print_bool(presenter.is_valid()),
//                 size                        = display_size(presenter.clone()),
//                 upnl                        = display_profit(presenter.percent_change(), presenter.unrealised_profit_usd()),
//                 rpnl                        = display_profit(presenter.percent_change(), presenter.realised_profit_usd()),
//                 entry_price                 = position.entry_price(),
//                 exit_price                  = position.exit_price().map_or("".to_string(), |m| format!("{:.8}", m)),
//                 time = presenter.position.trades.first().map(|trade| trade.time.format("%Y-%m-%d %H:%M").to_string()).unwrap_or("-".to_string()),
//                 normal_width = NORMAL_COLUMN_WIDTH,

//         wide_width = WIDE_COLUMN_WIDTH
//             )
//         }).collect::<Vec<String>>().join("\n"),
//         normal_width = NORMAL_COLUMN_WIDTH,

//         wide_width = WIDE_COLUMN_WIDTH
//     )
// }

fn display_size(presenter: PositionPresenter) -> String {
    format!(
        "{:.2} ({:.2} btc, ${:.2})",
        presenter.qty(),
        presenter.current_value_in_btc(),
        presenter.current_value_in_usd(),
    )
}

fn display_profit(percent_change: f64, unrealised_profit_usd: f64) -> colored::ColoredString {
    if unrealised_profit_usd == 0.0 {
        "".black()
    } else {
        positive_negative(
            percent_change,
            format!(
                "{} ({})",
                print_percent(percent_change),
                print_fiat(unrealised_profit_usd),
            ),
        )
    }
}

fn display_number_by_asset_type(value: f64, asset_type: AssetType) -> String {
    match asset_type {
        AssetType::Fiat => format!("${:.2}", value),
        AssetType::Bitcoin => format!("{:.8}", value),
        AssetType::Altcoin => format!("{:.8}", value),
    }
}
