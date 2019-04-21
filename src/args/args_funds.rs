use crate::args::*;
use crate::error::CliResult;
use cryptotrader::exchanges::ExchangeAPI;

pub fn parse_funds<E: ExchangeAPI + ?Sized>(matches: &ArgMatches, client: Box<E>) -> CliResult<String>
{
    let opening_balance: Option<f64> = matches
        .value_of("opening_balance")
        .and_then(|b| b.parse::<f64>().ok());

    let exchange = client.display().clone();
    let presenter = crate::commands::funds::fetch(client)?;

    Ok(crate::display::funds::ticker(&exchange, presenter, opening_balance))
}
