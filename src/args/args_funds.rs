use crate::args::*;

pub fn parse_funds<E>(matches: &ArgMatches, client: E) -> CliResult<String>
where
    E: ExchangeAPI,
{
    let opening_balance: Option<f64> = matches
        .value_of("opening_balance")
        .and_then(|b| b.parse::<f64>().ok());

    let presenter = commands::funds::fetch(client)?;

    Ok(display::funds::ticker(presenter, opening_balance))
}
