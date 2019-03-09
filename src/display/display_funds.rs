use crate::display::*;
use crate::utils::*;
use colored::*;
use cryptotrader;
use cryptotrader::models::*;
use cryptotrader::presenters::*;

pub fn ticker(presenter: BalancePresenter, opening_balance: Option<f64>) -> String {
    format!(
        "{}{}{}{}{}",
        "FUNDS".cyan(),
        presenter
            .total_value_in_btc()
            .map(|p| format!(" {:.4} btc", p))
            .unwrap_or("-".to_string()),
        presenter
            .total_value_in_usd()
            .map(|p| format!(" (${:.0})", p))
            .unwrap_or("-".to_string()),
        display_opening_balance(&presenter, opening_balance),
        find_first_btc_usd_pair(presenter.pairs.clone())
            .map(|btc_pair| format!(" :: {} ${}", "BTC PRICE".cyan(), btc_pair.price))
            .unwrap_or("".to_string())
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
            let current_balance = presenter.total_value_in_btc().unwrap_or(0.0);
            let profit_as_percent = price_percent(opening_balance, current_balance);
            let profit_btc = current_balance - opening_balance;
            let profit_usd = profit_btc
                * find_first_btc_usd_pair(presenter.pairs.clone())
                    .map(|p| p.price)
                    .unwrap_or(0.0);

            positive_negative(
                profit_as_percent,
                format!(" {} (${:.2})", print_percent(profit_as_percent), profit_usd),
            )
        })
        .unwrap_or("".normal())
}
