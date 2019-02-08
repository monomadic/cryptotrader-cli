use crate::display;
use crate::error::*;
use cryptotrader;
use cryptotrader::{exchanges::*, models::*};
use log::info;

pub fn fetch<E>(client: E, symbols: Vec<&str>) -> CliResult<String>
where
    E: ExchangeAPI,
{
    info!("client: all_pairs()");
    let pairs = client.all_pairs()?;
    info!("response: found {} pairs", pairs.len());

    let pairs = sort_pairs(pairs);

    // filter symbols
    let pairs = if symbols.len() > 0 {
        filter_pairmap_by_symbols(pairs, symbols)
    } else {
        pairs
    };

    Ok(display::pairs::table(pairs, client.base_pairs()))
}
