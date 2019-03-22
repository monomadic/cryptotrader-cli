use crate::args::*;

pub fn parse_verbose(matches: &clap::ArgMatches) {
    if matches.is_present("verbose") {
        let _ = simple_logger::init_with_level(log::Level::Info);
    }
}
