#![allow(dead_code)]
#![allow(unused_variables)]

// todo: rename to parse

use clap;
use clap::{load_yaml, AppSettings, ArgMatches};
use cryptotrader::{exchanges::binance::BinanceAPI, exchanges::ExchangeAPI};
use log::info;

use crate::display;
use crate::error::*;

pub fn parse() -> CliResult<String> {
    let yaml = load_yaml!("../cli.yml");
    let matches = clap::App::from_yaml(yaml)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    if matches.is_present("verbose") {
        let _ = simple_logger::init_with_level(log::Level::Info);
        info!("verbose logging enabled.");
    }

    // args have successfully parsed so we can start loading config etc.
    let conf = cryptotrader::config::read()?;
    let keys = &conf.exchange["binance"];
    let client = BinanceAPI::connect(&keys.api_key, &keys.secret_key);

    match matches.subcommand() {
        ("positions", Some(m)) => parse_positions(m, client),
        ("pairs", Some(m)) => parse_pairs(m, client),
        _ => Err(Box::new(CliError::InvalidCommand)),
    }
}

fn parse_positions<E>(matches: &ArgMatches, client: E) -> CliResult<String>
where
    E: ExchangeAPI,
{
    let positions = crate::commands::positions::fetch(client)?;

    Ok(if let Some(format) = matches.value_of("format") {
        match format {
            "table" => display::positions::table(positions),
            "ticker" => display::positions::ticker(positions),
            _ => display::positions::ticker(positions),
        }
    } else {
        display::positions::ticker(positions)
    })
}

fn parse_pairs<E>(_matches: &ArgMatches, client: E) -> CliResult<String>
where
    E: ExchangeAPI,
{
    crate::commands::pairs::fetch(client, vec!["BTC", "BNB", "LINK", "BTT"])
}
