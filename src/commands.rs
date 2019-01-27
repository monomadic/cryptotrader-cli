use crate::error::*;
use crate::utils::*;

use cryptotrader;
use cryptotrader::exchanges::ExchangeAPI;
use cryptotrader::models::*;
use cryptotrader::presenters::*;

// enum PositionStyle {
// 	Normal,
// 	Verbose,
// 	Ticker,
// }

pub fn positions<E>(client: E) -> CliResult<String> where E:ExchangeAPI {
	let pairs = vec!["NULSBTC".to_string(), "WAVESBTC".to_string()];

	let prices = client.prices()?;
	let btc_price_in_usd = *(prices.get("BTCUSDT").unwrap_or(&0.0));
	// let result_buffer = Vec::new();

	for pair in pairs.clone() {
		if let Ok(orders) = client.trades_for(&pair) {
			let current_price = *(prices.get(&pair).unwrap_or(&0.0));
			let grouped_orders = cryptotrader::models::group_by_price(orders.clone());
			let positions = Position::new(grouped_orders);

			if let Some(position) = positions.last() {
				let pp = PositionPresenter{ position: position.clone(), current_price, btc_price_in_usd };
				println!("PRICE: {:?}", pp);
			}
		}
	}
	


    Ok(iter_map(pairs, {|pair| format!("[{}]", pair)})
    	.join(" "))
}
