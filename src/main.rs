mod args;
mod commands;
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
