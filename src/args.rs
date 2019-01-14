use clap::{App, ArgMatches, AppSettings};

use crate::error::*;

pub fn parse() -> CliResult<&'static str> {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    if matches.is_present("verbose") {
        let _ = simple_logger::init_with_level(log::Level::Info);
        info!("verbose logging enabled.");
    }

    // args have successfully parsed so we can start loading config etc.
    let conf = cryptotrader::config::read(false)?;

    match matches.subcommand() {
        ("positions", Some(m)) => parse_positions(m),
        _ => { Err(Box::new(CliError::InvalidCommand)) },
    }
}

fn parse_positions(_matches: &ArgMatches) -> CliResult<&'static str> {
    super::commands::positions(["hi"].to_vec())
}
