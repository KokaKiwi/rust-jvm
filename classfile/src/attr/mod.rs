pub mod info;

use error::*;
use self::info::AttrInfo;
use super::constant::ConstantPool;

#[derive(Debug)]
pub struct Attr {
    name_index: usize,
    pub info: AttrInfo,
}

impl Attr {
    pub fn name<'a>(&self, pool: &'a ConstantPool) -> Option<&'a str> {
        pool.get_str(self.name_index)
    }
}

impl_read! {
    Attr(reader, pool: &ConstantPool) -> Result<Attr> = {
        // Read name index
        let name_index = try!(reader.read_u16::<BigEndian>()) as usize;
        let name = match pool.get_str(name_index) {
            Some(name) => name,
            None => bail!(ErrorKind::BadAttrName(name_index)),
        };

        // Read attr info
        let info = try!(AttrInfo::read(reader, name, pool));

        Ok(Attr {
            name_index: name_index,
            info: info,
        })
    }
}

impl_print! {
    Attr(self, printer, constant_pool: &ConstantPool) {
        let name = self.name(constant_pool).expect("Invalid name index");

        try!(printer.write_indent());
        try!(writeln!(printer, "Attr `{}`:", name));

        try!(self.info.print(&mut printer.sub_indent(1), constant_pool));
    }
}
