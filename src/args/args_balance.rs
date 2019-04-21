use crate::args::*;
use crate::error::CliResult;
use cryptotrader::exchanges::ExchangeAPI;

pub fn parse<E: ExchangeAPI + ?Sized>(matches: &ArgMatches, client: Box<E>) -> CliResult<String>
{
    let exchange = client.display().clone();
    let presenter = crate::commands::funds::fetch(client)?;

    Ok(crate::display::balance::ticker(&exchange, presenter))
}
