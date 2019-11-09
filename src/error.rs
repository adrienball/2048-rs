use std::boxed::Box;
use std::convert::Into;
use std::error;
use std::fmt;
use std::marker::{Send, Sync};

/// The error struct of game-2048
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    error: Box<dyn error::Error + Send + Sync>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ErrorKind {
    /// The board representation is invalid
    InvalidBoardRepr,
    /// The square value is invalid
    InvalidSquareValue(u16),
}

impl Error {
    /// Construct a new `Error` of a particular `ErrorKind`.
    pub fn new<E>(kind: ErrorKind, error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Error {
            kind,
            error: error.into(),
        }
    }

    /// Get the kind of this `Error`.
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.error.description()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.error.fmt(f)
    }
}
