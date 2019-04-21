use crate::error::*;
use cryptotrader;
use cryptotrader::exchanges::*;
use cryptotrader::presenters::BalancePresenter;

pub fn fetch<E: ExchangeAPI + ?Sized>(client: Box<E>) -> CliResult<BalancePresenter> {
    let prices = client.all_prices()?;
    let assets = client.balances()?;

    Ok(BalancePresenter { assets, prices })
}
