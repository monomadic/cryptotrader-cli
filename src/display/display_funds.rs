use crate::display::*;
use crate::utils::*;
// use colored::*;
use cryptotrader;
use cryptotrader::models::*;
use cryptotrader::presenters::*;

pub fn ticker(presenter: BalancePresenter, opening_balance: Option<f64>) -> String {
    format!(
        "{}{}{}\n{}{}",
        format!(
            "{} {:.4} btc",
            "FUNDS".cyan(),
            presenter.total_value_in_btc()
        ),
        display_opening_balance(&presenter, opening_balance),
        Price::find_first_btc_usd_price(&presenter.prices)
            .map(|btc_pair| format!(" :: {} ${}", "BTC PRICE".cyan(), btc_pair.price))
            .unwrap_or("".to_string()),
        "ALTS ".cyan(),
        format!(
            " {:.2} btc {:.0}%",
            presenter.alts_value_in_btc(),
            presenter.alts_value_in_btc() / presenter.total_value_in_btc() * 100.,
        ),
    )
}

fn display_opening_balance(
    presenter: &BalancePresenter,
    opening_balance: Option<f64>,
) -> ColoredString {
    opening_balance
        .map(|opening_balance_btc| {
            // let current_balance_btc = presenter.total_value_in_btc();
            let current_balance_usd = presenter.total_value_in_usd();
            let btc_price = Price::find_first_btc_usd_price(&presenter.prices)
                .map(|p| p.price)
                .unwrap_or(0.0);
            let opening_balance_usd = opening_balance_btc * btc_price;
            let profit_as_percent = price_percent(opening_balance_usd, current_balance_usd);
            // let profit_btc = current_balance_btc - opening_balance_btc;
            let profit_usd = current_balance_usd - opening_balance_usd;

            positive_negative(
                profit_as_percent,
                format!(" {} (${:.2})", print_percent(profit_as_percent), profit_usd),
            )
        })
        .unwrap_or("".normal())
}
