use crate::{ args::*, commands, display, error::* };
use cryptotrader::exchanges::ExchangeAPI;
use clap;

pub fn parse_orders<E: ExchangeAPI + ?Sized>(matches: &clap::ArgMatches, client: Box<E>) -> CliResult<String> {
    let orders = crate::commands::orders::fetch(client)?;
    Ok(display::orders::table(orders))
}
