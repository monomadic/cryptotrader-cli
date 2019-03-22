use crate::args::*;

pub fn parse_scan<E>(matches: &ArgMatches, client: E) -> CliResult<String>
where
    E: ExchangeAPI,
{
    crate::commands::scan::fetch(
        client,
        matches
            .values_of("pairs")
            .ok_or(CliError::ArgumentError("pairs not found".to_string()))?
            .collect(),
    )
}
