use crate::{args::*, commands, display};
use clap;

pub fn parse_orders<E>(matches: &clap::ArgMatches, client: E) -> CliResult<String>
where
    E: ExchangeAPI,
{
    let orders = commands::orders::fetch(client)?;
    Ok(display::orders::table(orders))
}
