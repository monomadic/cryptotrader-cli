use crate::error::*;
use crate::utils::*;
use cryptotrader;
use cryptotrader::{exchanges::*, models::*};
use log::info;

pub fn fetch<E>(client: E, symbol: &str) -> CliResult<Vec<Vec<Trade>>>
where
    E: ExchangeAPI,
{
    let pairs = client.all_pairs()?;
    let pairs = find_all_pairs_by_symbol(&symbol, pairs.clone());

    info!(
        "found pairs: {}",
        string_map(pairs.clone(), { |p| p.base }).join(" ")
    );

    // for pair in pairs {
    let trades: Vec<Vec<Trade>> = map_ok(
        pairs
            .into_iter()
            .map(|pair| {
                client._trades_for(&client.symbol_and_base_to_pair_format(&pair.symbol, &pair.base))
            })
            .collect(),
    );

    Ok(trades)
}
