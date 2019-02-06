use crate::error::*;

use cryptotrader;
use cryptotrader::{ exchanges::* };

pub fn pairs<E>(client: E) -> CliResult<String> where E:ExchangeAPI {
    let prices = client.all_pairs()?;

    Ok(format!("{:?}", prices))
}
