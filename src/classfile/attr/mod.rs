pub mod info;

use podio::{ReadPodExt, BigEndian};
use self::info::AttrInfo;
use std::io::{Read, Write};
use super::constant::ConstantPool;
use super::error::{Result, Error};
use utils::print::{Print, Printer};

#[derive(Debug)]
pub struct Attr {
    name_index: usize,
    pub info: AttrInfo,
}

impl Attr {
    pub fn read<R: Read>(reader: &mut R, cp: &ConstantPool) -> Result<Attr> {
        // Read name index
        let name_index = try!(reader.read_u16::<BigEndian>()) as usize;
        let name = match cp.get_str(name_index) {
            Some(name) => name,
            None => return Err(Error::BadAttrName(name_index)),
        };

        // Read attr info
        let info = try!(AttrInfo::read(reader, name));

        Ok(Attr {
            name_index: name_index,
            info: info,
        })
    }

    pub fn get_name<'a>(&self, pool: &'a ConstantPool) -> Option<&'a str> {
        pool.get_str(self.name_index)
    }
}

impl<'a> Print<&'a ConstantPool> for Attr {
    fn dump<W: Write>(&self, printer: &mut Printer<W, &'a ConstantPool>) -> ::std::io::Result<()> {
        let name = self.get_name(printer.context).unwrap();

        try!(printer.indent());
        try!(writeln!(printer, "Attr `{}`:", name));

        try!(printer.sub().with_indent(4).print(&self.info));

        Ok(())
    }
}
