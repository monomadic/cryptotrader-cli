#[macro_use]
extern crate clap;

use std::process;

mod args;
mod commands;
mod error;

fn main() {
    match args::parse() {
        Ok(s) => println!("{}", s),
        Err(e) => { println!("{}", e); process::exit(1); },
    }
}
