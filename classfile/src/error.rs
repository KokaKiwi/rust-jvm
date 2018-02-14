use std::io;
use super::constant;

error_chain! {
    links {
        ConstantPool(constant::error::Error, constant::error::ErrorKind);
    }

    foreign_links {
        Io(io::Error);
    }

    errors {
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
        BadTagValue(value: u8) {
            description("Bad tag value")
            display("Bad tag value: {:#x} `{}`", value, *value as char)
        }
    }
}
