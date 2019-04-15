use crate::error::*;
use cryptotrader;
use cryptotrader::exchanges::*;
use cryptotrader::presenters::BalancePresenter;

pub fn fetch<E>(client: E) -> CliResult<BalancePresenter>
where
    E: ExchangeAPI,
{
    let prices = client.all_prices()?;
    let assets = client.balances()?;

    Ok(BalancePresenter { assets, prices })
}
