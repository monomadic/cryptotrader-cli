use cryptotrader;
use cryptotrader::{ models::* };

use colored::*;

type PairMap = HashMap<String, Vec<Pair>>;
use std::collections::HashMap;

pub fn table(pairs: PairMap) -> String {
    pairs.into_iter().map(|(symbol, pairs)| {
        let pairs = pairs.into_iter().map(|p| format!("{} {}", p.base, p.price)).collect::<Vec<String>>().join(" : ");
        format!("{:.10} - {}\n", symbol.yellow(), pairs)
    }).collect::<Vec<String>>().join("")
}
