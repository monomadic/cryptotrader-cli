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
        Pair::find_first_btc_usd_pair(&presenter.pairs)
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
        .map(|opening_balance| {
            // let btc_price = find_first_btc_usd_pair(&presenter.pairs.clone())
            //     .map(|p| p.price)
            //     .unwrap_or(0.0);
            let current_balance = presenter.total_value_in_btc();
            let profit_as_percent = price_percent(opening_balance, current_balance);
            let profit_btc = current_balance - opening_balance;
            let profit_usd = profit_btc
                * Pair::find_first_btc_usd_pair(&presenter.pairs)
                    .map(|p| p.price)
                    .unwrap_or(0.0);

            positive_negative(
                profit_as_percent,
                format!(" {} (${:.2})", print_percent(profit_as_percent), profit_usd),
            )
        })
        .unwrap_or("".normal())
}
