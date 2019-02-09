use cryptotrader;
use cryptotrader::models::*;
use super::*;
// use colored::*;

type PairMap = HashMap<String, Vec<Pair>>;
use std::collections::HashMap;

pub fn table(pairs: PairMap, base_pairs: Vec<String>) -> String {
    let mut output_buffer = format!("{:16}", "");

    output_buffer.push_str(
        &base_pairs
            .clone()
            .into_iter()
            .map(|symbol| format!("{:16}", symbol.yellow()))
            .collect::<Vec<String>>()
            .join(""),
    );

    output_buffer.push_str("\n");

    output_buffer.push_str(
        &pairs
            .into_iter()
            .map(|(symbol, pairs)| {
                format!(
                    "{:16}{}",
                    symbol.yellow(),
                    base_pairs
                        .clone()
                        .into_iter()
                        .map(|base_pair| {
                            if let Some(pair) =
                                pairs.clone().into_iter().find(|p| p.base == base_pair)
                            {
                                format!("{:<16}", pair.price)
                            } else {
                                format!("{:<16}", "-")
                            }
                        })
                        .collect::<Vec<String>>()
                        .join("")
                )
            })
            .collect::<Vec<String>>()
            .join("\n"),
    );

    output_buffer
}

// pub fn pretty_price(pair: &Pair) -> String {
//     // if is_tiny_number(pair.price) {
//     //     format!("{:.8}", pair.price)
//     // } else {
//     //     format!("{:.3}", pair.price)
//     // }
//     match pair.base.as_ref() {
//         "BTC" => format!("{:.8}", pair.price),
//         "USDT" => format!("${:.3}", pair.price),
//         _ => format!("{}", pair.price),
//     }
// }

pub fn pretty_price_from_base(base: &str, cost: f64) -> String {
    match base {
        "BTC" => format!("{:.8}", cost),
        "USDT" => format!("${:.2}", cost),
        _ => format!("{:.3}", cost),
    }
}
