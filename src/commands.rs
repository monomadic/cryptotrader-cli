use crate::error::*;

// todo: rename to process or fetch

use cryptotrader;
use cryptotrader::{
	exchanges::ExchangeAPI,
	models::*,
	presenters::*,
};

mod pairs; pub use self::pairs::*;

pub fn positions<E>(client: E) -> CliResult<String> where E:ExchangeAPI {
	use crate::display::DisplayTicker;

	let funds = client.funds()?;
	let prices = client.all_pairs()?;
	let btcusd_pair = client.btc_price()?;

	let total_value_in_btc = 50.0;
	let pairs:Vec<String> = funds.alts.clone().into_iter().map(|fund| format!("{}BTC", fund.symbol)).collect();
	let mut result_buffer = Vec::new();

	for pair in pairs.clone() {
		if let Ok(orders) = client.trades_for(&pair) {
			if let Some(current_pair) = prices.clone().into_iter().find(|p| p.symbol == pair && p.base == "BTC") {
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
							current_price: current_pair.price,
							btc_price_in_usd: btcusd_pair.price,
							wallet_qty,
						}.display_ticker()
					);
				}
			}
		}
	}

	let mut result = result_buffer.join(" :: ");

	use colored::*;
	result.push_str(&format!("\n{} {:.4} btc :: {} ${:.0}",
		"FUNDS".cyan(),
		total_value_in_btc,
		"BTC PRICE".cyan(),
		btcusd_pair.price,
	));
	
	Ok(result)
}
