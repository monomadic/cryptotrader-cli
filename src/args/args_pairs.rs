use crate::args::*;
use crate::error::*;
use cryptotrader::exchanges::ExchangeAPI;

pub fn parse_pairs<E:ExchangeAPI + ?Sized>(_matches: &ArgMatches, client:Box<E>) -> CliResult<String> {
    crate::commands::pairs::fetch(client, vec!["BTC", "BNB", "LINK", "BTT"])
}
