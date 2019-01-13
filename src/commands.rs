use crate::error::*;

pub fn positions(_pairs: Vec<&str>) -> CliResult<&'static str> {
    Ok("positions")
}
