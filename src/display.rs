use colored::*;
use cryptotrader::presenters::*;

pub trait DisplayTicker {
    fn display_ticker(&self) -> String;
}

impl DisplayTicker for PositionPresenter {
    fn display_ticker(&self) -> String {
        format!("{} {:.2} btc {} {}",
            self.symbol().yellow(),
            self.current_value_in_btc(),
            positive_negative(self.percent_change(), format!("{:.2}%", self.percent_change())),
            positive_negative(self.unrealised_profit_usd(), format!("(${:.2})", self.unrealised_profit_usd())),
        )
    }
}

fn positive_negative(number: f64, string: String) -> String {
    if number > 0.01 {
        string.green().to_string()
    } else {
        if number < 0.01 {
            string.red().to_string()
        } else {
            string
        }
    }
}
