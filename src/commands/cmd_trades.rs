use crate::error::*;
use crate::utils::*;
use cryptotrader;
use cryptotrader::presenters::TradePresenter;
use cryptotrader::{exchanges::*, models::*};
use log::info;

pub fn fetch<E: ExchangeAPI + ?Sized>(
    client: Box<E>,
    symbol: &str,
    limit: Option<usize>,
    group: bool,
) -> CliResult<Vec<TradePresenter>> {
//    let pairs = client.all_pairs()?;
    let prices = client.all_prices()?;

    let fiat_pair = client.btc_usd_pair();
    let mut presenters: Vec<TradePresenter> = Vec::new();

//    info!("{:?}", pairs);

//    info!(
//        "found pairs: {}",
//        string_map(pairs.clone(), { |p| p.base }).join(" ")
//    );

//    for pair in pairs.clone() {
        let trades = client.trades_for_pair(Pair::from_string(symbol))?;
        let trades = match group {
            true => group_and_average_trades_by_trade_type(trades),
            false => trades,
        };
        let trades = optional_limit(limit, trades);

        for trade in trades {
            presenters.push(TradePresenter {
                trade: trade.clone(),
                fiat_pair: fiat_pair.clone(),
                price_in_trade_currency: prices.price_for(trade.pair.clone()).ok_or(CliError::PairNotFound(trade.pair.to_string()))?
            })
        }
//    }

    // sort by date
    presenters.sort_by(|a, b| a.trade.time.cmp(&b.trade.time));

    Ok(presenters)
}
