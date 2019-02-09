use super::*;
use cryptotrader::presenters::*;

pub fn ticker(presenters: Vec<PositionPresenter>) -> String {
    presenters
        .into_iter()
        .map(|presenter| {
            format!(
                "{} {}{:.2} btc {} {}",
                presenter.symbol().yellow(),
                print_bool(presenter.is_valid()),
                presenter.current_value_in_btc(),
                positive_negative(
                    presenter.percent_change(),
                    format!("{:.2}%", presenter.percent_change())
                ),
                positive_negative(
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
        "{:12}{:1}{:9}{:50}{:50}{:16}{:16}\n{}",
        "SYMBOL", " ", "SIZE", "UPNL", "RPNL", "ENTRY", "EXIT",
        &presenters.into_iter().map(|presenter| {
            let position = presenter.position.clone();

            format!("{symbol:12}{valid:1}{size:<32}{upnl:50}{rpnl:<50}{entry_price:<16.8}{exit_price:<16}",
                symbol                      = position.symbol().yellow(),
                valid                       = print_bool(presenter.is_valid()),
                size                        = format!("{:.2} ({:.2} btc, ${:.2})", presenter.position.remaining_qty(), presenter.current_value_in_btc(), presenter.current_value_in_usd()),
                upnl                        = positive_negative(presenter.percent_change(), format!("{:.2}%", presenter.percent_change())),
                rpnl                        = positive_negative(presenter.unrealised_profit_usd(), format!("(${:.2})", presenter.unrealised_profit_usd())),
                entry_price                 = position.entry_price(),
                exit_price                  = position.exit_price().map_or("".to_string(), |m| format!("{:.8}", m)),
            )
        }
    ).collect::<Vec<String>>().join("\n"))
}
