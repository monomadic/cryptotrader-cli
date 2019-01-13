use clap::{App, ArgMatches, AppSettings};

use crate::error::*;

pub fn parse() -> CliResult<&'static str> {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    match matches.subcommand() {
        // ("positions", Some(m)) => run_analyse(m, &logger),
        ("positions", Some(m)) => parse_positions(m),
        _ => { Err(Box::new(CliError::InvalidCommand)) },
    }
}

fn parse_positions(_matches: &ArgMatches) -> CliResult<&'static str> {
    super::commands::positions(["hi"].to_vec())
}
