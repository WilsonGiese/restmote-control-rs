use rustc_serialize::json;

use rustful;

use std::io;
use std::fmt;

#[derive(Debug)]
pub enum RcError {
    Parser(json::ParserError),
    Io(io::Error),
    Server(rustful::HttpError),
    Config(String),
}

impl From<io::Error> for RcError {
    fn from(result: io::Error) -> RcError {
        RcError::Io(result)
    }
}

impl From<json::ParserError> for RcError {
    fn from(result: json::ParserError) -> RcError {
        RcError::Parser(result)
    }
}

impl From<rustful::HttpError> for RcError {
    fn from(result: rustful::HttpError) -> RcError {
        RcError::Server(result)
    }
}

impl fmt::Display for RcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RcError::Parser(ref e) => write!(f, "{}", e),
            RcError::Io(ref e) => write!(f, "{}", e),
            RcError::Config(ref e) => write!(f, "{}", e),
            RcError::Server(ref e) => write!(f, "{}", e), 
        }
    }
}
