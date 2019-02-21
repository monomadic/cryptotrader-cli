use crate::error::*;
use cryptotrader;
use cryptotrader::{exchanges::*, models::*, presenters::*};
use log::info;

pub fn fetch<E>(client: E) -> CliResult<Vec<Vec<TradePresenter>>>
where
    E: ExchangeAPI,
{
    let assets = client.balances()?;
    let pairs = client.all_pairs()?;

    let mut result_buffer = Vec::new();
    for asset in assets {
        if let Some(btc_pair_for_asset) =
            find_pair_by_symbol_and_base(&asset.symbol, &client.btc_symbol(), pairs.clone())
        {
            if asset.amount >= 1.0 {
                let trades = client.trades_for_pair(btc_pair_for_asset.clone())?;

                // combine and average into BUY-SELL-BUY-SELL array
                let trades = group_and_average_trades_by_trade_type(trades);

                // let fiat_pair = client.fiat_pair_for(&asset.symbol, pairs.clone());
                let fiat_pair = client.btc_pair(pairs.clone());

                // filter out the SELLs
                let trades: Vec<Trade> = trades
                    .into_iter()
                    .filter(|trade| trade.trade_type == TradeType::Buy)
                    .collect();

                // take the most recent (open) trade
                if let Some(trade) = trades.last().map(|t| t.clone()) {
                    // let symbol_pairs = find_all_pairs_by_symbol(&asset.symbol, pairs.clone());

                    result_buffer.push(vec![TradePresenter { trade, fiat_pair }]);
                }
            }
        }
    }

    info!("trade presenters: {:#?}", result_buffer);
    Ok(result_buffer)
}

// pub fn fetch<E>(client: E) -> CliResult<Vec<PositionPresenter>>
// where
//     E: ExchangeAPI,
// {
//     info!("client: balances()");
//     let assets = client.balances()?;
//     info!(
//         "response: found assets: {}",
//         assets
//             .clone()
//             .into_iter()
//             .map(|p| p.symbol)
//             .collect::<Vec<String>>()
//             .join(", ")
//     );

//     let pairs = client.all_pairs()?;

//     let mut result_buffer = Vec::new();
//     for asset in assets {
//         if let Some(btc_pair_for_asset) =
//             find_pair_by_symbol_and_base(&asset.symbol, &client.btc_symbol(), pairs.clone())
//         {
//             let trades = client.trades_for_pair(btc_pair_for_asset)?;
//             let positions = Position::new(trades, asset.amount);

//             if let Some(position) = positions.last() {
//                 let symbol_pairs = find_all_pairs_by_symbol(&asset.symbol, pairs.clone());
//                 result_buffer.push(PositionPresenter::new(position.clone(), symbol_pairs));
//             }
//         }
//     }

//     info!("position presenters: {:#?}", result_buffer);
//     Ok(result_buffer)
// }
