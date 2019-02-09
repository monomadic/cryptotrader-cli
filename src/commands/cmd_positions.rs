use crate::error::*;
use cryptotrader;
use cryptotrader::{exchanges::*, models::*, presenters::*};
use log::info;

pub fn fetch<E>(client: E) -> CliResult<Vec<PositionPresenter>>
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

    info!("client: all_pairs()");
    let pairs = client.all_pairs()?;
    info!("response: found {} pairs.", pairs.len());

    let mut result_buffer = Vec::new();
    for asset in assets {
        if let Some(btc_pair_for_asset) =
            find_pair_by_symbol_and_base(&asset.symbol, &client.btc_symbol(), pairs.clone())
        {
            let pair = &client.pair_format(btc_pair_for_asset);
            info!("client: trades_for({})", pair);
            let orders = client.trades_for(pair)?;
            let grouped_orders = cryptotrader::models::group_by_price(orders.clone());
            let positions = Position::new(grouped_orders); // fix this to give one order, take multiple positions

            if let Some(position) = positions.last() {
                let symbol_pairs = find_all_pairs_by_symbol(&asset.symbol, pairs.clone());
                result_buffer.push(PositionPresenter::new(position.clone(), symbol_pairs));
            }
        }
    }

    Ok(result_buffer)
}
