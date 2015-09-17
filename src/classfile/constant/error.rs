use std::convert::From;
use std::io;
use std::result;
use std::string::FromUtf8Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BadTagValue(u8),
    Utf8Error(FromUtf8Error),
    Io(io::Error),
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::Utf8Error(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}
