use crate::error::*;
use cryptotrader;
use cryptotrader::{exchanges::*, models::*, presenters::*};
use log::info;

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
                let trade_pairs = Trade::group_by_trade_pair(trades);

                if let Some(most_recent_trade_pair) = trade_pairs.last() {
                    let pairs = Pair::base_pairs_for_symbol(&asset.symbol, &pairs);

                    if let Ok(position) = Position::new(most_recent_trade_pair.to_vec(), asset) {
                        result_buffer.push(PositionPresenter {
                            position,
                            pairs,
                            btc_price_in_usd,
                        });
                    }
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
