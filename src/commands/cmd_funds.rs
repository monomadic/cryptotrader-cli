use crate::error::*;
use cryptotrader;
use cryptotrader::exchanges::*;
use cryptotrader::models::*;
use cryptotrader::presenters::*;

pub fn fetch<E: ExchangeAPI + ?Sized>(client: Box<E>) -> CliResult<BalancePresenter> {
    let prices = client.all_prices()?;
    let assets = client.balances()?.filter_small_balances(0.1);

    Ok(BalancePresenter { assets, prices })
}
