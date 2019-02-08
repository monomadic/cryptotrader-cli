use cryptotrader::presenters::*;
use super::*;

pub fn ticker(presenters: Vec<PositionPresenter>) -> String {
    presenters.into_iter().map(|presenter|
        format!("{} {}{:.2} btc {} {}",
            presenter.symbol().yellow(),
            print_bool(presenter.is_valid()),
            presenter.current_value_in_btc(),
            positive_negative(presenter.percent_change(), format!("{:.2}%", presenter.percent_change())),
            positive_negative(presenter.unrealised_profit_usd(), format!("(${:.2})", presenter.unrealised_profit_usd())),
        )
    ).collect::<Vec<String>>().join(" :: ")
}

pub fn table(presenters: Vec<PositionPresenter>) -> String {
    presenters.into_iter().map(|presenter|
        format!("{} {}{:.2} btc {} {}",
            presenter.symbol().yellow(),
            print_bool(presenter.is_valid()),
            presenter.current_value_in_btc(),
            positive_negative(presenter.percent_change(), format!("{:.2}%", presenter.percent_change())),
            positive_negative(presenter.unrealised_profit_usd(), format!("(${:.2})", presenter.unrealised_profit_usd())),
        )
    ).collect::<Vec<String>>().join("\n")
}
