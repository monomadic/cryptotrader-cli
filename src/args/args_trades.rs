use crate::args::args_format::parse_format;
use crate::args::*;
use crate::error::*;
use cryptotrader::exchanges::ExchangeAPI;

pub fn parse_trades<E:ExchangeAPI + ?Sized>(matches: &clap::ArgMatches, client: Box<E>) -> CliResult<String> {
    let limit = matches
        .value_of("limit")
        .and_then(|b| b.parse::<usize>().ok());

    if let Some(symbol) = matches.value_of("symbol") {
        let trades = crate::commands::trades::fetch(client, symbol, limit, matches.is_present("group"))?;

        Ok(crate::display::trades::table(trades))
    } else {
        panic!("symbol required");
    }
}
