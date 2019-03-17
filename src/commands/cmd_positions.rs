use crate::error::*;
use cryptotrader;
use cryptotrader::{exchanges::*, models::*, presenters::*};
use log::info;

// pub fn fetch<E>(client: E) -> CliResult<Vec<Vec<TradePresenter>>>
// where
//     E: ExchangeAPI,
// {
//     let assets = client.balances()?;
//     let pairs = client.all_pairs()?;

//     let mut result_buffer = Vec::new();
//     for asset in assets {
//         if let Some(btc_pair_for_asset) =
//             find_pair_by_symbol_and_base(&asset.symbol, &client.btc_symbol(), pairs.clone())
//         {
//             if asset.amount >= 1.0 {
//                 let trades = client.trades_for_pair(btc_pair_for_asset.clone())?;

//                 // combine and average into BUY-SELL-BUY-SELL array
//                 let trades = group_and_average_trades_by_trade_type(trades);

//                 // let fiat_pair = client.fiat_pair_for(&asset.symbol, pairs.clone());
//                 let fiat_pair = client.btc_pair(pairs.clone());

//                 // filter out the SELLs
//                 let trades: Vec<Trade> = trades
//                     .into_iter()
//                     .filter(|trade| trade.trade_type == TradeType::Buy)
//                     .collect();

//                 // take the most recent (open) trade
//                 if let Some(trade) = trades.last().map(|t| t.clone()) {
//                     // let symbol_pairs = Pair::base_pairs_for_symbol(&asset.symbol, &pairs);

//                     result_buffer.push(vec![TradePresenter { trade, fiat_pair }]);
//                 }
//             }
//         }
//     }

//     info!("trade presenters: {:#?}", result_buffer);
//     Ok(result_buffer)
// }

pub fn fetch<E>(client: E, pairs: Option<Vec<Pair>>) -> CliResult<Vec<PositionPresenter>>
where
    E: ExchangeAPI,
{
    info!("client: balances()");
    let assets = client.balances()?;
    info!(
        "response: found assets: {}",
        assets
            .clone()
            .into_iter()
            .map(|p| p.symbol)
            .collect::<Vec<String>>()
            .join(", ")
    );

    let pairs = pairs.unwrap_or(client.all_pairs()?);

    let btc_price_in_usd = client
        .btc_pair(pairs.clone())
        .map(|p| p.price)
        .unwrap_or(0.0);

    let mut result_buffer = Vec::new();
    for asset in assets {
        if asset.asset_type() == AssetType::Altcoin {
            if asset.amount >= 1.0 {
                let trades = client.trades_for_symbol(&asset.symbol, pairs.clone())?;
                let position = Position::new(trades, asset.clone());
                let pairs = Pair::base_pairs_for_symbol(&asset.symbol, &pairs);

                if let Ok(position) = position {
                    result_buffer.push(PositionPresenter {
                        position,
                        pairs,
                        btc_price_in_usd,
                    });
                }
            }
        }
    }

    // result_buffer.sort_by(|a, b| a.time().cmp(&b.time()));
    result_buffer.sort_by(|a, b| {
        b.unrealised_profit_usd()
            .partial_cmp(&a.unrealised_profit_usd())
            .unwrap_or(std::cmp::Ordering::Less)
    });

    info!("position presenters: {:?}", result_buffer);
    Ok(result_buffer)
}
