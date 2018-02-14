use std::convert::From;
use std::string::FromUtf8Error;

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Utf8(FromUtf8Error);
    }

    errors {
        BadTagValue(value: u8) {
            description("Bad tag value")
            display("Bad tag value: {:x}", value)
        }
    }
}
