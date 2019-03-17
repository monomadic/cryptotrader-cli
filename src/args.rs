#![allow(dead_code)]
#![allow(unused_variables)]

// todo: rename to parse

use clap;
use clap::{load_yaml, AppSettings, ArgMatches};
use cryptotrader::models::Pair;
use cryptotrader::{exchanges::binance::BinanceAPI, exchanges::ExchangeAPI};

use crate::commands;
use crate::display;
use crate::error::*;

pub fn parse() -> CliResult<String> {
    let yaml = load_yaml!("../cli.yml");
    let matches = clap::App::from_yaml(yaml)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    parse_verbose(&matches);

    // args have successfully parsed so we can start loading config etc.
    let conf = cryptotrader::config::read()?;
    let keys = &conf.exchange["binance"];
    let client = BinanceAPI::connect(&keys.api_key, &keys.secret_key);

    match matches.subcommand() {
        ("positions", Some(m)) => parse_positions(m, client),
        ("pairs", Some(m)) => parse_pairs(m, client),
        ("trades", Some(m)) => parse_trades(m, client),
        ("orders", Some(m)) => parse_orders(m, client),
        ("funds", Some(m)) => parse_funds(m, client),
        ("scan", Some(m)) => parse_scan(m, client),
        _ => Err(Box::new(CliError::ArgumentError(
            "Invalid Argument".to_string(),
        ))),
    }
}

fn parse_verbose(matches: &ArgMatches) {
    if matches.is_present("verbose") {
        let _ = simple_logger::init_with_level(log::Level::Info);
    }
}

enum DisplayFormat {
    Ticker,
    Table,
    Default,
}

fn parse_format(matches: &ArgMatches) -> DisplayFormat {
    if let Some(format) = matches.value_of("format") {
        match format {
            "table" => DisplayFormat::Table,
            "ticker" => DisplayFormat::Ticker,
            _ => DisplayFormat::Default,
        }
    } else {
        DisplayFormat::Default
    }
}

fn parse_orders<E>(matches: &ArgMatches, client: E) -> CliResult<String>
where
    E: ExchangeAPI,
{
    let orders = commands::orders::fetch(client)?;
    Ok(display::orders::table(orders))
}

fn parse_funds<E>(matches: &ArgMatches, client: E) -> CliResult<String>
where
    E: ExchangeAPI,
{
    let opening_balance: Option<f64> = matches
        .value_of("opening_balance")
        .and_then(|b| b.parse::<f64>().ok());

    let presenter = commands::funds::fetch(client)?;

    Ok(display::funds::ticker(presenter, opening_balance))
}

fn parse_positions<E>(matches: &ArgMatches, client: E) -> CliResult<String>
where
    E: ExchangeAPI,
{
    let pairs: Option<Vec<Pair>> = matches.values_of("pairs").map(|p| {
        commands::pairs::parse_pairs(
            &client,
            p.into_iter().map(|pair| pair.to_string()).collect(),
        )
    });

    let positions = commands::positions::fetch(client, pairs)?;
    let show_trades = matches.is_present("show-trades");

    Ok(match parse_format(matches) {
        DisplayFormat::Table => display::positions::table(positions, show_trades),
        DisplayFormat::Ticker => display::positions::ticker(positions),
        DisplayFormat::Default => display::positions::table(positions, show_trades),
    })
}

fn parse_pairs<E>(_matches: &ArgMatches, client: E) -> CliResult<String>
where
    E: ExchangeAPI,
{
    crate::commands::pairs::fetch(client, vec!["BTC", "BNB", "LINK", "BTT"])
}

fn parse_scan<E>(matches: &ArgMatches, client: E) -> CliResult<String>
where
    E: ExchangeAPI,
{
    crate::commands::scan::fetch(
        client,
        matches
            .values_of("pairs")
            .ok_or(CliError::ArgumentError("pairs not found".to_string()))?
            .collect(),
    )
}

fn parse_trades<E>(matches: &ArgMatches, client: E) -> CliResult<String>
where
    E: ExchangeAPI,
{
    let limit = matches
        .value_of("limit")
        .and_then(|b| b.parse::<usize>().ok());

    if let Some(symbol) = matches.value_of("symbol") {
        let trades = commands::trades::fetch(client, symbol, limit, matches.is_present("group"))?;

        Ok(match parse_format(matches) {
            DisplayFormat::Table => display::trades::table(trades),
            DisplayFormat::Ticker => display::trades::ticker(trades),
            DisplayFormat::Default => display::trades::table(trades),
        })
    } else {
        panic!("symbol required");
    }
}

// fn parse_limit(limit: Option<&str>) -> Option<usize> {
//     limit.map(|l| l.parse::<usize>().ok()).unwrap_or(None)
// }
