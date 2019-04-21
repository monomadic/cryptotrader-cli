#![allow(dead_code)]
#![allow(unused_variables)]

use crate::error::*;
use clap;
use clap::{load_yaml, AppSettings, ArgMatches};
use cryptotrader::{exchanges::binance::BinanceAPI, exchanges::ExchangeAPI};
use cryptotrader::exchanges::huobi::HuobiAPI;

mod args_format;
mod args_funds;
mod args_orders;
mod args_pairs;
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
    let keys = &conf.exchange[matches.value_of("exchange").unwrap_or("binance")];

    let client:Box<dyn ExchangeAPI> = match matches.value_of("exchange") {
        Some("binance") => Box::new(BinanceAPI::new(&keys.api_key, &keys.secret_key)),
        Some("huobi") => Box::new(HuobiAPI::new(&keys.api_key, &keys.secret_key)?),
        _ => { return Err(Box::new(CliError::ArgumentError("no exchange argument found".to_string())))}
    };

    match matches.subcommand() {
//        ("positions", Some(m)) => args_positions::parse_positions(m, client),
        ("pairs", Some(m)) => args_pairs::parse_pairs(m, client),
        ("trades", Some(m)) => args_trades::parse_trades(m, client),
        ("orders", Some(m)) => args_orders::parse_orders(m, client),
        ("funds", Some(m)) => args_funds::parse_funds(m, client),
        _ => Ok("no cmd".to_string()),
    }
}
