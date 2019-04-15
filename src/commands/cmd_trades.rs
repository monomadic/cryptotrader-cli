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
) -> CliResult<Vec<TradePresenter>>
where
    E: ExchangeAPI,
{
    let pairs = client.all_pairs()?;
    let fiat_pair = client.btc_usd_pair();
    let pairs = Pair::base_pairs_for_symbol(&symbol, &pairs);
    let mut presenters: Vec<TradePresenter> = Vec::new();

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

        for trade in trades {
            presenters.push(TradePresenter {
                trade,
                fiat_pair: fiat_pair.clone(),
            })
        }
    }

    // sort by date
    presenters.sort_by(|a, b| a.trade.time.cmp(&b.trade.time));

    Ok(presenters)
}
