use colored::*;

pub mod display_pairs;
pub use self::display_pairs as pairs;
//pub mod display_positions;
//pub use self::display_positions as positions;
pub mod display_assets;
pub use self::display_assets as assets;
pub mod display_trades;
pub use self::display_trades as trades;
pub mod display_trade_type;
pub use self::display_trade_type as trade_type;
pub mod display_funds;
pub use self::display_funds as funds;
pub mod display_orders;
pub use self::display_orders as orders;

static SMALL_COLUMN_WIDTH: usize = 8;
static NORMAL_COLUMN_WIDTH: usize = 16;
static WIDE_COLUMN_WIDTH: usize = 32;

use prettytable::format::TableFormat;
fn table_format() -> TableFormat {
    prettytable::format::FormatBuilder::new()
        .column_separator(' ')
        .borders(' ')
        // .separators(
        //     &[LinePosition::Top, LinePosition::Bottom],
        //     LineSeparator::new('▬', '▬', '●', '●'),
        // )
        .padding(1, 1)
        .build()
}

fn print_bool(condition: bool) -> String {
    match condition {
        true => "".to_string(),
        false => "*".red().to_string(),
    }
}

// TODO: make private
pub fn positive_negative(number: f64, string: String) -> ColoredString {
    if number > 0.01 {
        string.green()
    } else if number < -0.01 {
        string.red()
    } else {
        string.normal()
    }
}

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

// fn print_btc(num: f64) -> String {
//     if num < 1.0 {
//         format!("{:.8}", num)
//     } else {
//         format!("{:.4}", num)
//     }
// }
