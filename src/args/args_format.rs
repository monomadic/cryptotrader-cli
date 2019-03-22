use crate::args::*;

pub fn parse_format(matches: &clap::ArgMatches) -> DisplayFormat {
    if let Some(format) = matches.value_of("format") {
        match format {
            "table" => DisplayFormat::Table,
            "ticker" => DisplayFormat::Ticker,
            _ => DisplayFormat::Default,
        }
    } else {
        DisplayFormat::Default
    }
}
