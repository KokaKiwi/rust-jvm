pub mod classfile;
pub mod code;
pub mod field;
pub mod method;
pub mod misc;

use podio::{ReadPodExt, BigEndian};
use std::io::{Read, Write};
use classfile::error::Result;
use classfile::constant::ConstantPool;
use utils::print::{Print, Printer};

#[derive(Debug)]
pub enum AttrInfo {
    // Class file

    // Code

    // Field
    ConstantValue(field::ConstantValueAttrInfo),

    // Method
    Code(method::CodeAttrInfo),

    // Misc

    // Unknown
    Unknown {
        name: String,
        data: Vec<u8>,
    },
}

impl AttrInfo {
    pub fn read<R: Read>(reader: &mut R, name: &str) -> Result<AttrInfo> {
        use std::io::Cursor;
        use utils::io::ReadExt;

        let size = try!(reader.read_u32::<BigEndian>()) as usize;
        let data = try!(reader.read_vec(size));

        {
            let mut reader: Cursor<&[u8]> = Cursor::new(&data);

            match name {
                // Class file

                // Code

                // Field
                "ConstantValueAttrInfo" => return field::ConstantValueAttrInfo::read(&mut reader).map(AttrInfo::ConstantValue),

                // Method
                "Code" => return method::CodeAttrInfo::read(&mut reader).map(AttrInfo::Code),

                // Misc

                // Unknown
                _ => {}
            }
        }

        Ok(AttrInfo::Unknown {
            name: name.to_string(),
            data: data,
        })
    }
}

impl<'a> Print<&'a ConstantPool> for AttrInfo {
    fn dump<W: Write>(&self, printer: &mut Printer<W, &'a ConstantPool>) -> ::std::io::Result<()> {
        match *self {
            // Class file

            // Code

            // Field
            AttrInfo::ConstantValue(ref info) => info.dump(printer),

            // Method
            AttrInfo::Code(ref info) => info.dump(printer),

            // Misc

            // Unknown
            AttrInfo::Unknown { name: _, ref data } => {
                try!(printer.indent());
                try!(writeln!(printer, "Unknown: [ {} bytes... ]", data.len()));
                Ok(())
            }
        }
    }
}
