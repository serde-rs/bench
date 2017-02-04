use serde::{ser, de};
use serde::de::value;
use std::{self, error, io, result, string};
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Error;

pub type Result<T> = std::result::Result<T, Error>;

impl error::Error for Error {
    fn description(&self) -> &str {
        "error"
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "error")
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(_: T) -> Self {
        Error
    }
}

impl de::Error for Error {
    fn custom<T: Display>(_: T) -> Self {
        Error
    }
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Self {
        Error
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(_: string::FromUtf8Error) -> Self {
        Error
    }
}

impl From<value::Error> for Error {
    fn from(_: value::Error) -> Self {
        Error
    }
}
