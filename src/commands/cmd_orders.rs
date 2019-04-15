use crate::error::*;
use cryptotrader;
use cryptotrader::{exchanges::*, models::*};

pub fn fetch<E>(client: E) -> CliResult<Vec<Order>>
where
    E: ExchangeAPI,
{
    let pairs = client.all_pairs()?;
    let orders = client.open_orders()?;

    Ok(orders)
}
