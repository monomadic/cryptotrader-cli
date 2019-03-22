use crate::args::*;

pub fn parse_pairs<E>(_matches: &ArgMatches, client: E) -> CliResult<String>
where
    E: ExchangeAPI,
{
    crate::commands::pairs::fetch(client, vec!["BTC", "BNB", "LINK", "BTT"])
}
