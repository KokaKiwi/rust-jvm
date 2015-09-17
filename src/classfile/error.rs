use std::convert::From;
use std::io;
use std::result;
use super::constant;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BadAccessFlags(u16),
    BadAttrName(usize),
    BadMagicValue(u32),
    ConstantPoolError(constant::Error),
    Io(io::Error),
}

impl From<constant::Error> for Error {
    fn from(err: constant::Error) -> Error {
        Error::ConstantPoolError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}
