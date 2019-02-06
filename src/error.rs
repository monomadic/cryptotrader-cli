#![allow(dead_code)]

use core::fmt::Display;
use std::fmt;
use std::error::Error;

pub type CliResult<T> = Result<T,Box<std::error::Error>>;

#[derive(Debug, Clone)]
pub enum CliError {
    InvalidCommand,
    PairNotFound(String),
}

impl Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.clone() {
            CliError::InvalidCommand => write!(f, "Invalid command."),
            CliError::PairNotFound(pair) => write!(f, "Pair not found: {}", pair),
        }
    }
}

impl Error for CliError {
    fn description(&self) -> &str { "Cli Error." }
}
