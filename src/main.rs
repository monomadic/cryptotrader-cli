mod args;
mod commands {
    pub mod cmd_pairs; pub use self::cmd_pairs as pairs;
    pub mod cmd_positions; pub use self::cmd_positions as positions;
}
mod error;
mod utils;
mod display;

use std::process;

fn main() {
    match args::parse() {
        Ok(s) => println!("{}", s),
        Err(e) => { println!("error: {}", e); process::exit(1); },
    }
}
