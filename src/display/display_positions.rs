use super::*;
use cryptotrader::presenters::*;
use log::info;

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

pub fn table(presenters: Vec<PositionPresenter>) -> String {
    format!(
        "{:12}{:1}{:32}{:50}{:50}{:16}{:16}\n{}",
        "SYMBOL", " ", "SIZE", "UPNL", "RPNL", "ENTRY", "EXIT",
        &presenters.into_iter().map(|presenter| {
            let position = presenter.position.clone();
            info!("table({})", position.symbol());

            format!("{symbol:12}{valid:1}{size:<32}{upnl:50}{rpnl:<50}{entry_price:<16.8}{exit_price:<16}",
                symbol                      = position.symbol().yellow(),
                valid                       = print_bool(presenter.is_valid()),
                size                        = size(presenter.clone()),
                upnl                        = positive_negative(presenter.percent_change(), format!("{:.2}%", presenter.percent_change())),
                rpnl                        = positive_negative(presenter.unrealised_profit_usd(), format!("(${:.2})", presenter.unrealised_profit_usd())),
                entry_price                 = position.entry_price(),
                exit_price                  = position.exit_price().map_or("".to_string(), |m| format!("{:.8}", m)),
            )
        }).collect::<Vec<String>>().join("\n")
    )
}

fn size(presenter: PositionPresenter) -> String {
    format!(
        "{:.2} ({:.2} btc, ${:.2})",
        presenter.qty(),
        presenter.current_value_in_btc(),
        presenter.current_value_in_usd(),
    )
}
