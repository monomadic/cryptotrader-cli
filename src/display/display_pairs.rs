use super::*;
use cryptotrader;
use cryptotrader::models::*;
use prettytable::{cell, Row, Table};

type PairMap = HashMap<String, Vec<Pair>>;
use std::collections::HashMap;

pub fn display_pair(pair: &Pair) -> String {
    format!("{}-{}", pair.symbol, pair.base)
}

fn format_pair(pair: Pair, base_pair: &str) -> String {
    let btc_price_in_usd = 4000.0;
    format!(
        "{:<32}",
        match AssetType::from_symbol(&base_pair) {
            AssetType::Fiat => format!("${:.2} ({:.8})", pair.price, pair.price / btc_price_in_usd),
            AssetType::Bitcoin => {
                format!("{:.8} (${:.2})", pair.price, pair.price * btc_price_in_usd)
            }
            AssetType::Altcoin => {
                format!("{:.8} (${:.2})", pair.price, pair.price * btc_price_in_usd)
            }
        }
    )
}

fn pair_row(symbol: &str, pairs: Vec<Pair>, base_pairs: &Vec<String>) -> Row {
    let mut row = Row::new(Vec::new());
    row.add_cell(cell!(symbol.yellow()));

    for base_pair in base_pairs {
        row.add_cell(cell!(if let Some(pair) =
            pairs.clone().into_iter().find(|p| &p.base == base_pair)
        {
            format_pair(pair, base_pair)
        } else {
            format!("{:<32}", "-")
        }));
    }

    row
}

pub fn table(pairs: PairMap, base_pairs: Vec<String>) -> String {
    // let mut output_buffer = format!("{:16}", "");
    // let btc_price_in_usd = 4000.0;

    let mut table = Table::new();
    table.set_format(table_format());

    // use prettytable::row;
    // table.set_titles(row!(base_pairs
    //     .iter()
    //     .map(|p| &p)
    //     .collect::Vec<&str>()
    //     .to_array()));

    for pair in pairs.clone() {
        table.add_row(pair_row(&pair.0, pair.1, &base_pairs));
    }

    // let mut rows: Vec<Cell> = vec![Cell::new("")];

    // rows.push(
    //     base_pairs
    //         .clone()
    //         .into_iter()
    //         .map(|symbol| Cell::new(&format!("{:32}", symbol.yellow())))
    //         .collect(),
    // );

    // table.add_row(Row::new(rows));

    // table.printstd();

    // output_buffer.push_str(
    //     &base_pairs
    //         .clone()
    //         .into_iter()
    //         .map(|symbol| format!("{:32}", symbol.yellow()))
    //         .collect::<Vec<String>>()
    //         .join(""),
    // );

    // output_buffer.push_str("\n");

    // output_buffer.push_str(
    //     &pairs
    //         .into_iter()
    //         .map(|(symbol, pairs)| {
    //             format!(
    //                 "{:16}{}",
    //                 symbol.yellow(),
    //                 base_pairs
    //                     .clone()
    //                     .into_iter()
    //                     .map(|base_pair| {
    //                         if let Some(pair) =
    //                             pairs.clone().into_iter().find(|p| p.base == base_pair)
    //                         {
    //                             format!(
    //                                 "{:<32}",
    //                                 match AssetType::from_symbol(&base_pair) {
    //                                     AssetType::Fiat => format!(
    //                                         "${:.2} ({:.8})",
    //                                         pair.price,
    //                                         pair.price / btc_price_in_usd
    //                                     ),
    //                                     AssetType::Bitcoin => format!(
    //                                         "{:.8} (${:.2})",
    //                                         pair.price,
    //                                         pair.price * btc_price_in_usd
    //                                     ),
    //                                     AssetType::Altcoin => format!(
    //                                         "{:.8} (${:.2})",
    //                                         pair.price,
    //                                         pair.price * btc_price_in_usd
    //                                     ),
    //                                 }
    //                             )
    //                         } else {
    //                             format!("{:<32}", "-")
    //                         }
    //                     })
    //                     .collect::<Vec<String>>()
    //                     .join("")
    //             )
    //         })
    //         .collect::<Vec<String>>()
    //         .join("\n"),
    // );

    // output_buffer

    table.to_string()
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
