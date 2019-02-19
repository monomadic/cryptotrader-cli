use crate::error::*;
use crate::utils::*;
use cryptotrader;
use cryptotrader::presenters::TradePresenter;
use cryptotrader::{exchanges::*, models::*};
use log::info;

pub fn fetch<E>(
    client: E,
    symbol: &str,
    limit: Option<usize>,
    group: bool,
) -> CliResult<Vec<Vec<TradePresenter>>>
where
    E: ExchangeAPI,
{
    let pairs = client.all_pairs()?;
    let fiat_pair = client.btc_pair(pairs.clone());
    let pairs = find_all_pairs_by_symbol(&symbol, pairs.clone());
    let mut all_trades: Vec<Vec<TradePresenter>> = Vec::new();

    info!(
        "found pairs: {}",
        string_map(pairs.clone(), { |p| p.base }).join(" ")
    );

    for pair in pairs.clone() {
        let trades = client.trades_for_pair(pair)?;
        let trades = match group {
            true => group_and_average_trades_by_trade_type(trades),
            false => trades,
        };
        let trades = optional_limit(limit, trades);

        let presenters = trades
            .into_iter()
            .map(|trade| TradePresenter {
                trade,
                fiat_pair: fiat_pair.clone(),
            })
            .collect();

        all_trades.push(presenters);
    }

    Ok(all_trades)
}
