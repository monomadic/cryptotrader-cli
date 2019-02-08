use colored::*;

pub mod display_pairs; pub use self::display_pairs as pairs;
pub mod display_positions; pub use self::display_positions as positions;
pub mod display_assets; pub use self::display_assets as assets;

fn print_bool(condition: bool) -> String {
    match condition {
        true    => "".to_string(),
        false   => "*".red().to_string(),
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
