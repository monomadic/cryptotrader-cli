#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;
extern crate simple_logger;

extern crate cryptotrader;

use std::process;

mod args;
mod commands;
mod error;
mod utils;

fn main() {
    match args::parse() {
        Ok(s) => println!("{}", s),
        Err(e) => { println!("error: {}", e); process::exit(1); },
    }
}
