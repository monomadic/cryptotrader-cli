use crate::error::*;

use cryptotrader;
use cryptotrader::exchanges::ExchangeAPI;
use cryptotrader::models::*;
use cryptotrader::presenters::*; 

pub fn positions<E>(client: E) -> CliResult<String> where E:ExchangeAPI {
	use crate::display::DisplayTicker;

	// let pairs = vec!["NULSBTC".to_string(), "WAVESBTC".to_string()];
	let funds = client.funds()?;
	let prices = client.prices()?;

	let btc_price_in_usd = *(prices.get("BTCUSDT").unwrap_or(&0.0));
	let funds_presenter = FundsPresenter::new(funds.clone(), prices.clone(), btc_price_in_usd.clone());
	let pairs:Vec<String> = funds.alts.clone().into_iter().map(|fund| format!("{}BTC", fund.symbol)).collect();
	let mut result_buffer = Vec::new();

	for pair in pairs.clone() {
		if let Ok(orders) = client.trades_for(&pair) {
			let current_price = *(prices.get(&pair).unwrap_or(&0.0));
			let grouped_orders = cryptotrader::models::group_by_price(orders.clone());
			let positions = Position::new(grouped_orders); // fix this to give one order, take multiple positions

			if let Some(position) = positions.last() {
				let assets:Vec<Asset> = funds.alts.clone().into_iter()
					.filter(|asset| position.symbol().contains(&asset.symbol))
					.collect();

				let wallet_qty = assets.last()
					.expect(&format!("no qty found for: {}", position.symbol()))
					.amount;

				let position = position.clone();

				result_buffer.push(
					PositionPresenter{
						position,
						current_price,
						btc_price_in_usd,
						wallet_qty,
					}.display_ticker()
				);
			}
		}
	}

	let mut result = result_buffer.join(" :: ");

	use colored::*;
	result.push_str(&format!("\n{} {:.4} btc :: {} ${:.0}",
		"FUNDS".cyan(),
		funds_presenter.total_value_in_btc,
		"BTC PRICE".cyan(),
		btc_price_in_usd,
	));
	
	Ok(result)

    // Ok(iter_map(pairs, {|pair| format!("[{}]", pair)})
    // 	.join(" "))
}
