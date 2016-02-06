use byteorder;
use std::convert::From;
use std::io;
use std::result;
use super::constant;

pub type Result<T> = result::Result<T, Error>;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        BadAccessFlags(flags: u16) {
            description("Bad access flags")
            display("Bad access flags: {:#x}", flags)
        }
        BadAttrName(value: usize) {
            description("Bad attribute name")
        }
        BadMagicValue(value: u32) {
            description("Bad magic value")
            display("Bad magic value: {:#x}", value)
        }
        ByteOrderError(err: byteorder::Error) {
            cause(err)
            description(err.description())
            display("Byteorder error: {}", err)
            from()
        }
        ConstantPoolError(err: constant::Error) {
            cause(err)
            description(err.description())
            display("Constant error: {}", err)
            from()
        }
        Io(err: io::Error) {
            cause(err)
            description(err.description())
            display("I/O error: {}", err)
            from()
        }
    }
}
