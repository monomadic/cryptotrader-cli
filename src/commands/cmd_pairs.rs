use crate::display;
use crate::error::*;
use cryptotrader;
use cryptotrader::{exchanges::*, models::*};

pub fn fetch<E: ExchangeAPI + ?Sized>(client: Box<E>, symbols: Vec<&str>) -> CliResult<String>
where
    E: ExchangeAPI,
{
    let prices = client.all_prices()?;
//    let prices = sort_prices(prices);
    Ok(display::pairs::table(prices))
}

pub fn parse_pairs<E>(client: &E, pairs: Vec<String>) -> Vec<Pair>
where
    E: ExchangeAPI,
{
    pairs
        .into_iter()
        .filter_map(|p|parse_pair(p).ok())
        .collect()
}

pub fn parse_pair(pair: String) -> CliResult<Pair> {
    let arg_error = CliError::ArgumentError("Pairs must take the form SYM_BASE or SYM-BASE.".to_string());

    let split_char = if pair.contains("_") { "_"} else if pair.contains("-") { "-" } else {
        return Err(Box::new(arg_error))
    };

    let mut pairs = pair.split_terminator(split_char);
    let symbol = pairs.next().ok_or(arg_error.clone())?.to_string();
    let base = pairs.next().ok_or(arg_error)?.to_string();

    Ok(Pair { symbol, base })
}
