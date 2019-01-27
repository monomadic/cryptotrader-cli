use crate::error::*;
use crate::utils::*;
use crate::display;

use cryptotrader;
use cryptotrader::exchanges::ExchangeAPI;
use cryptotrader::models::*;
use cryptotrader::presenters::*;

pub fn positions<E>(client: E) -> CliResult<String> where E:ExchangeAPI {
	use crate::display::DisplayTicker;

	// let pairs = vec!["NULSBTC".to_string(), "WAVESBTC".to_string()];
	let funds = client.funds()?;
	let pairs:Vec<String> = funds.alts.into_iter().map(|fund| format!("{}BTC", fund.symbol)).collect();

	let prices = client.prices()?;
	let btc_price_in_usd = *(prices.get("BTCUSDT").unwrap_or(&0.0));
	let mut result_buffer = Vec::new();

	for pair in pairs.clone() {
		if let Ok(orders) = client.trades_for(&pair) {
			let current_price = *(prices.get(&pair).unwrap_or(&0.0));
			let grouped_orders = cryptotrader::models::group_by_price(orders.clone());
			let positions = Position::new(grouped_orders); // fix this to give one order, take multiple positions

			if let Some(position) = positions.last() {
				result_buffer.push(
					PositionPresenter{
						position: position.clone(),
						current_price,
						btc_price_in_usd
					}.display_ticker()
				);
			}
		}
	}
	
	Ok(result_buffer.join(" :: "))

    // Ok(iter_map(pairs, {|pair| format!("[{}]", pair)})
    // 	.join(" "))
}
