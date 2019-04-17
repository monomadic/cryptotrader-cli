#![allow(dead_code)]
#![allow(unused_variables)]

use crate::error::*;
use crate::{commands, display};
use clap;
use clap::{load_yaml, AppSettings, ArgMatches};
//use cryptotrader::models::Pair;
use cryptotrader::{exchanges::binance::BinanceAPI, exchanges::ExchangeAPI};
use cryptotrader::exchanges::Exchange::Huobi;
use cryptotrader::exchanges::huobi::HuobiAPI;

mod args_format;
mod args_funds;
mod args_orders;
mod args_pairs;
//mod args_positions;
mod args_trades;
mod args_verbose;

pub enum DisplayFormat {
    Ticker,
    Table,
    Default,
}

pub fn parse() -> CliResult<String> {
    let yaml = load_yaml!("../cli.yml");
    let matches = clap::App::from_yaml(yaml)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    args_verbose::parse_verbose(&matches);

    // args have successfully parsed so we can start loading config etc.
    let conf = cryptotrader::config::read()?;

    let keys = &conf.exchange["huobi"];
    let client = HuobiAPI::new(&keys.api_key, &keys.secret_key)?;

//    let keys = &conf.exchange["binance"];
//    let client = BinanceAPI::new(&keys.api_key, &keys.secret_key);

    match matches.subcommand() {
//        ("positions", Some(m)) => args_positions::parse_positions(m, client),
        ("pairs", Some(m)) => args_pairs::parse_pairs(m, client),
        ("trades", Some(m)) => args_trades::parse_trades(m, client),
        ("orders", Some(m)) => args_orders::parse_orders(m, client),
        ("funds", Some(m)) => args_funds::parse_funds(m, client),
        _ => Ok("no cmd".to_string()),
    }
}
