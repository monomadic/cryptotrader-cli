use crate::error::*;

use cryptotrader;
use cryptotrader::{ exchanges::* };

pub fn pairs<E>(client: E) -> CliResult<String> where E:ExchangeAPI {
    let prices = client.all_pairs()?;

    let result = prices.into_iter().map(|pair| {
        format!("{}-{} {}", pair.symbol, pair.base, pair.price)
    }).collect::<Vec<String>>().join(" :: ");

    Ok(result)
}
