use core::fmt::Display;
use std::fmt;
use std::error::Error;

pub type CliResult<T> = Result<T,Box<std::error::Error>>;

#[derive(Debug)]
pub enum CliError {
    InvalidCommand,
}

impl Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::InvalidCommand => write!(f, "Invalid command."),
            // _ => write!(f, ""),
        }
    }
}

impl Error for CliError {
    fn description(&self) -> &str { "error" }

    // fn source(&self) -> Option<&(dyn Error + 'static)> {
    //     Some(&self.side)
    // }
}
