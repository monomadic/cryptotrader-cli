use crate::args::args_format::parse_format;
use crate::args::*;

pub fn parse_trades<E>(matches: &clap::ArgMatches, client: E) -> CliResult<String>
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
