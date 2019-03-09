use crate::display::positive_negative;
use crate::error::*;
use crate::utils::*;
use colored::*;
use cryptotrader::{
    exchanges::*, indicators::bbands::*, indicators::rsi::*, indicators::sma::*, models::*,
};

pub fn fetch<E>(client: E, pairs: Vec<&str>) -> CliResult<String>
where
    E: ExchangeAPI,
{
    Ok(pairs
        .iter()
        .map(|pair| strategy_rob(&client, pair))
        .collect::<Vec<String>>()
        .join("\n"))
}

// pub fn strategy_basic<E>(client: E, pair: &str) -> CliResult<String>
// where
//     E: ExchangeAPI,
// {
//     let candlesticks = client.chart_data(pair, "1h");
//     Ok(if let Ok(candlesticks) = candlesticks {
//         let rsi = last_rsi_value(&candlesticks);
//         let sma200 = *sma(200, &candlesticks).last().unwrap_or(&0.0);
//         let sma99 = *sma(99, &candlesticks).last().unwrap_or(&0.0);
//         let percent_sma = price_percent(sma99, sma200);

//         format!(
//             "{} RSI: {:.0} SMA200: {:.8} SMA99: {:.8} ({:.2}%)",
//             pair, rsi, sma200, sma99, percent_sma
//         )
//     } else {
//         "-".to_string()
//     })
// }

pub fn strategy_rob<E>(client: &E, pair: &str) -> String
where
    E: ExchangeAPI,
{
    let candlesticks = client.chart_data(pair, "1h");
    if let Ok(candlesticks) = candlesticks {
        use cryptotrader::indicators::macd::*;

        let macd = macd(12, 26, 9, &candlesticks).expect("candlestikcs");

        println!("MACD SCAN: {:?}", score(find_macd_crosses(macd)));

        let tests: String = vec![
            test_rsi_under(&candlesticks, 60.1).to_string(),
            test_above_sma(&candlesticks, 99, 200).to_string(),
            test_bbands(&candlesticks).to_string(),
        ]
        .join("\n");
        format!("{} 1h:\n{}", pair.yellow(), tests)
    } else {
        "-".to_string()
    }
}

// pub fn strategy_cj<E>(client: &E, pair: &str) -> String
// where
//     E: ExchangeAPI,
// {
//     // let sma99 = *sma(99, &candlesticks).last().unwrap_or(&0.0);
//     // let price: f64 = *candlesticks
//     //     .iter()
//     //     .map(|c| c.close_price)
//     //     .collect::<Vec<f64>>()
//     //     .last()
//     //     .unwrap_or(&0.0);
//     // let percent_sma99 = price_percent(sma99, price);

//     let candlesticks = client.chart_data(pair, "12h");
//     if let Ok(candlesticks) = candlesticks {
//         let tests: String = vec![test_last_golden_cross(&candlesticks).to_string()].join("\n");
//         format!("{} 12h:\n{}", pair.yellow(), tests)
//     } else {
//         "-".to_string()
//     }
// }

fn test_bbands(candlesticks: &Vec<Candlestick>) -> TestResult {
    if let Some(last_candlestick) = candlesticks.last() {
        if let Some(band) = bbands(20, &candlesticks).last() {
            return TestResult {
                display: format!(
                    "PRICE_ABOVE_LOWER_BOLLINGER_BAND: {:.8}, {:.8} {:?}",
                    last_candlestick.close_price, band.lower_band, band
                ),
                passfail: band.lower_band < last_candlestick.close_price,
            };
        }
    }
    TestResult {
        display: format!("test input failure"),
        passfail: false,
    }
}

fn test_above_sma(candlesticks: &Vec<Candlestick>, sma_fast: u32, sma_slow: u32) -> TestResult {
    let sma_fast_value = *sma(sma_fast, &candlesticks).last().unwrap_or(&0.0);
    let sma_slow_value = *sma(sma_slow, &candlesticks).last().unwrap_or(&0.0);
    let price_percent = price_percent(sma_slow_value.into(), sma_fast_value.into());

    TestResult {
        display: format!(
            "SMA{}/{}: {}",
            sma_fast,
            sma_slow,
            positive_negative(price_percent, format!("{:.2}%", price_percent))
        ),
        passfail: sma_fast_value > sma_slow_value,
    }
}

fn test_rsi_under(candlesticks: &Vec<Candlestick>, value: f64) -> TestResult {
    let rsi = last_rsi_value(&candlesticks);

    TestResult {
        display: format!("RSI: {:.0}", rsi),
        passfail: rsi < value,
    }
}

fn test_last_golden_cross(candlesticks: &Vec<Candlestick>) -> TestResult {
    let last_cross = last_golden_cross_sma99_sma200(&candlesticks, 99, 200);
    let display = last_cross
        .map(|candles_ago| format!("Golden cross {} candles ago.", candles_ago))
        .unwrap_or("No recent golden cross on this time frame.".to_string());
    let passfail = last_cross.is_some();
    // let weight = 10.0;
    // let score = 100.0;

    TestResult {
        display,
        passfail,
        // weight,
        // score,
    }
}

pub fn last_golden_cross_sma99_sma200(
    candlesticks: &Vec<Candlestick>,
    sma1: u32,
    sma2: u32,
) -> Option<usize> {
    let sma99s: Vec<f64> = sma(sma1, &candlesticks).into_iter().rev().collect();
    let sma200s: Vec<f64> = sma(sma2, &candlesticks).into_iter().rev().collect();

    for (index, sma99) in sma99s.into_iter().enumerate() {
        if let Some(sma200) = sma200s.get(index) {
            let price_percent = price_percent(*sma200, sma99).abs();

            if price_percent < 0.1 {
                return Some(index);
            }
        }
    }

    None
}

struct TestResult {
    display: String,
    passfail: bool,
    // weight: f64,
    // score: f64,
}

impl std::fmt::Display for TestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let passfail = match self.passfail {
            true => "PASS".green(),
            false => "FAIL".red(),
        };

        write!(f, "TEST [{}]: {}", passfail, self.display)
    }
}

// struct Strategy {
//     pub fn plays: Vec<StrategyPlay>,
// }

// struct StrategyPlay {
//     pub entry_bar: u32,
//     pub entry_price: f64,
//     pub exit_bar: u32,
//     pub exit_price: f64,
//     pub profit_as_percent: f64, // fn
// }
