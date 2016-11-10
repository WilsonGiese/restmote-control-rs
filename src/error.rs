use rustc_serialize::json;

use std;
use std::error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum RcError {
    Parser(json::ParserError),
    Io(std::io::Error),
    Config(String),
}

impl From<std::io::Error> for RcError {
    fn from(result: std::io::Error) -> RcError {
        RcError::Io(result)
    }
}

impl From<json::ParserError> for RcError {
    fn from(result: json::ParserError) -> RcError {
        RcError::Parser(result)
    }
}

impl Display for RcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RcError::Parser(ref e) => write!(f, "{}", e),
            RcError::Io(ref e) => write!(f, "{}", e),
            RcError::Config(ref e) => write!(f, "{}", e),
        }
    }
}
