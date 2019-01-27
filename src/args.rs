#![allow(dead_code)]
#![allow(unused_variables)]

// use cryptotrader::config::APIConfig;
use clap;

use crate::error::*;

pub fn parse() -> CliResult<String> {
    let yaml = load_yaml!("../cli.yml");
    let matches = clap::App::from_yaml(yaml)
        // .setting(AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    if matches.is_present("verbose") {
        let _ = simple_logger::init_with_level(log::Level::Info);
        info!("verbose logging enabled.");
    }

    // args have successfully parsed so we can start loading config etc.
    let conf = cryptotrader::config::read()?;
    let keys = &conf.exchange["binance"];
    let client = cryptotrader::exchanges::binance::connect(&keys.api_key, &keys.secret_key);

    super::commands::positions(client)

    // match matches.subcommand() {
    //     ("positions", Some(m)) => parse_positions(m, conf.exchange["binance"].clone()),
    //     _ => { Err(Box::new(CliError::InvalidCommand)) },
    // }
}

// fn parse_positions(_matches: &ArgMatches, _conf: APIConfig) -> CliResult<String> {
//     super::commands::positions()
// }
