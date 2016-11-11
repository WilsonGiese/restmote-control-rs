use rustc_serialize::json;

use rustful;

use std::io;
use std::fmt;

#[derive(Debug)]
pub enum RcError {
    Decoder(json::DecoderError),
    Io(io::Error),
    Server(rustful::HttpError),
    Config(String),
}

impl From<io::Error> for RcError {
    fn from(result: io::Error) -> RcError {
        RcError::Io(result)
    }
}

impl From<json::DecoderError> for RcError {
    fn from(result: json::DecoderError) -> RcError {
        RcError::Decoder(result)
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
            RcError::Decoder(ref e) => write!(f, "{}", e),
            RcError::Io(ref e) => write!(f, "{}", e),
            RcError::Config(ref e) => write!(f, "{}", e),
            RcError::Server(ref e) => write!(f, "{}", e),
        }
    }
}
