#![allow(dead_code)]
#![allow(unused_variables)]

pub fn iter_map<T, F>(vec: Vec<T>, method: F) -> Vec<T>
where
    F: Fn(T) -> T,
{
    vec.into_iter().map(method).collect()
}

pub fn string_map<T, F>(vec: Vec<T>, method: F) -> Vec<String>
where
    F: Fn(T) -> String,
{
    vec.into_iter().map(method).collect()
}

pub fn map_ok<T, E>(results: Vec<Result<T, E>>) -> Vec<T>
where
    E: ::std::fmt::Debug,
{
    results
        .into_iter()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect()
}

pub fn optional_limit<T>(limit: Option<usize>, vector: Vec<T>) -> Vec<T> {
    if let Some(limit) = limit {
        vector
            .into_iter()
            .rev()
            .take(limit)
            .collect::<Vec<T>>()
            .into_iter()
            .rev()
            .collect()
    } else {
        vector
    }
}

/// Expresses the difference as a percentage between two floats.
///
/// ```rust
/// use cryptotrader::presenters::price_percent;
/// assert_eq!(price_percent(5.0, 10.0), 100.0);
/// assert_eq!(price_percent(100.0, 50.0), -50.0);
/// ```
pub fn price_percent(entry_price: f64, exit_price: f64) -> f64 {
    if entry_price < exit_price {
        (100. / entry_price * exit_price) - 100.
    } else {
        -(100. + -100. / entry_price * exit_price)
    }
}
