use super::*;
use cryptotrader;
use cryptotrader::models::*;
use prettytable::{cell, Row, Table};

type PairMap = HashMap<String, Vec<Pair>>;
use std::collections::HashMap;

pub fn display_pair(pair: &Pair) -> String {
    format!("{}-{}", pair.symbol, pair.base)
}

// todo: fix this!
fn format_price(price: Price) -> String {
    let btc_price_in_usd = 4000.0;
    let current_price = price.price;

    format!(
        "{:<32}",
        match AssetType::from_symbol(&price.pair.symbol) {
            AssetType::Stablecoin => format!("${:.2} ({:.8})", current_price, current_price / btc_price_in_usd),
            AssetType::Bitcoin => {
                format!("{:.8} (${:.2})", current_price, current_price * btc_price_in_usd)
            }
            AssetType::Altcoin => {
                format!("{:.8} (${:.2})", current_price, current_price * btc_price_in_usd)
            }
        }
    )
}

fn price_row(symbol: &str, prices: Vec<Price>, base_pairs: &Vec<String>) -> Row {
    let mut row = Row::new(Vec::new());
    row.add_cell(cell!(symbol.yellow()));

    for base_pair in base_pairs {
        row.add_cell(cell!(if let Some(price) =
            prices.clone().into_iter().find(|p| &p.pair.base == base_pair)
        {
            format_price(price)
        } else {
            format!("{:<32}", "-")
        }));
    }

    row
}

pub fn table(prices: Vec<Price>, base_pairs: Vec<String>) -> String {
    let mut table = Table::new();
    table.set_format(table_format());

    for price in prices.clone() {
        table.add_row(price_row(&price.pair.symbol, prices.clone(), &base_pairs));
    }

    table.to_string()
}

pub fn pretty_price_from_base(base: &str, cost: f64) -> String {
    match base {
        "BTC" => format!("{:.8}", cost),
        "USDT" => format!("${:.2}", cost),
        _ => format!("{:.3}", cost),
    }
}
