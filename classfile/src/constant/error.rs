use std::convert::From;
use std::string::FromUtf8Error;

pub use error::Result;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        BadTagValue(value: u8) {
            description("Bad tag value")
            display("Bad tag value: {:x}", value)
        }
        Utf8Error(err: FromUtf8Error) {
            cause(err)
            description(err.description())
            display("UTF-8 error: {}", err)
            from()
        }
    }
}
