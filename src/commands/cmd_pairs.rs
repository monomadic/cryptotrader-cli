use crate::error::*;

use cryptotrader;
use cryptotrader::{ exchanges::*, models::* };

pub fn fetch<E>(client: E) -> CliResult<String> where E:ExchangeAPI {
    let pairs = client.all_pairs()?;
    let pairs = sort_pairs(pairs);

    Ok(crate::display::pairs::table(pairs))
}
