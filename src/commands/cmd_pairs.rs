use crate::display;
use crate::error::*;
use cryptotrader;
use cryptotrader::{exchanges::*, models::*};

pub fn fetch<E>(client: E, symbols: Vec<&str>) -> CliResult<String>
where
    E: ExchangeAPI,
{
    let pairs = client.all_pairs()?;
    let pairs = sort_pairs(pairs);

    // filter symbols
    let pairs = if symbols.len() > 0 {
        filter_pairmap_by_symbols(pairs, symbols)
    } else {
        pairs
    };

    Ok(display::pairs::table(pairs, client.base_pairs()))
}

pub fn parse_pairs<E>(client: &E, pairs: Vec<String>) -> Vec<Pair>
where
    E: ExchangeAPI,
{
    pairs
        .into_iter()
        .map(|pair| client.string_to_pair(pair.to_string(), 0.0))
        .filter_map(|e| e)
        .collect()
}
