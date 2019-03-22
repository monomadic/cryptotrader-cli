use crate::args::args_format::parse_format;
use crate::args::*;

pub fn parse_positions<E>(matches: &clap::ArgMatches, client: E) -> CliResult<String>
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
