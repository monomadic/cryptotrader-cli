mod args;
mod commands {
    pub mod cmd_pairs;
    pub use self::cmd_pairs as pairs;
    pub mod cmd_positions;
    pub use self::cmd_positions as positions;
    pub mod cmd_trades;
    pub use self::cmd_trades as trades;
    pub mod cmd_funds;
    pub use self::cmd_funds as funds;
    pub mod cmd_orders;
    pub use self::cmd_orders as orders;
}
mod display;
mod error;
mod utils;

use std::process;

fn main() {
    match args::parse() {
        Ok(s) => println!("{}", s),
        Err(e) => {
            println!("error: {}", e);
            process::exit(1);
        }
    }
}
