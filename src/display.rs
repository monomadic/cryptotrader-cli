use colored::*;

pub mod display_pairs;
pub use self::display_pairs as pairs;
pub mod display_positions;
pub use self::display_positions as positions;
pub mod display_assets;
pub use self::display_assets as assets;
pub mod display_trades;
pub use self::display_trades as trades;
pub mod display_trade_type;
pub use self::display_trade_type as trade_type;

fn print_bool(condition: bool) -> String {
    match condition {
        true => "".to_string(),
        false => "*".red().to_string(),
    }
}

fn positive_negative(number: f64, string: String) -> String {
    if number > 0.01 {
        string.green().to_string()
    } else if number < 0.01 {
        string.red().to_string()
    } else {
        string
    }
}

// fn is_tiny_number(num: f64) -> bool {
//     num < 1.0
// }

fn print_percent(num: f64) -> String {
    format!("{:.2}%", num)
}

fn print_fiat(num: f64) -> String {
    if num < 10.0 {
        format!("${:.3}", num)
    } else {
        format!("${:.0}", num)
    }
}

fn print_btc(num: f64) -> String {
    if num < 1.0 {
        format!("{:.8}", num)
    } else {
        format!("{:.4}", num)
    }
}