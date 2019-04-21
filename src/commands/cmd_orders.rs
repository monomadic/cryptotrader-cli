use crate::error::*;
use cryptotrader;
use cryptotrader::{exchanges::*, models::*};

pub fn fetch<E: ExchangeAPI + ?Sized>(client: Box<E>) -> CliResult<Vec<Order>> {
    let pairs = client.all_pairs()?;
    let orders = client.open_orders()?;

    Ok(orders)
}
