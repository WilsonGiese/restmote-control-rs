use rustc_serialize::json;

use std;
use std::error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum RcError {
    Parser(json::ParserError),
    Io(std::io::Error),
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
