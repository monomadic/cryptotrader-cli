use crate::error::*;
use crate::utils::*;
use cryptotrader;
use cryptotrader::{exchanges::*, models::*};
use log::info;

pub fn fetch<E>(
    client: E,
    symbol: &str,
    limit: Option<usize>,
    group: bool,
) -> CliResult<(Vec<Vec<Trade>>, Pair)>
where
    E: ExchangeAPI,
{
    let pairs = client.all_pairs()?;

    let btc_pair = client
        .btc_pair(pairs.clone())
        .expect(&format!("{:#?} BTC pair to be found.", pairs.clone()));

    let pairs = find_all_pairs_by_symbol(&symbol, pairs.clone());

    info!(
        "found pairs: {}",
        string_map(pairs.clone(), { |p| p.base }).join(" ")
    );

    let mut all_trades: Vec<Vec<Trade>> = Vec::new();

    for pair in pairs.clone() {
        let trades = client.trades_for_pair(pair)?;
        let trades = match group {
            true => group_and_average_trades_by_trade_type(trades),
            false => trades,
        };
        let trades = optional_limit(limit, trades);

        all_trades.push(trades);
    }

    info!("trades: {:#?}", all_trades);

    Ok((all_trades, btc_pair))
}
