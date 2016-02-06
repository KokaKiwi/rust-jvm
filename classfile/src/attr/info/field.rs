use constant::{ConstantPool, ConstantPoolEntry};
use error::Result;
use byteorder::{ReadBytesExt, BigEndian};
use std::io::Read;

#[derive(Debug)]
pub struct ConstantValueAttrInfo {
    value_index: usize,
}

impl ConstantValueAttrInfo {
    pub fn read<R: Read>(reader: &mut R, _pool: &ConstantPool) -> Result<ConstantValueAttrInfo> {
        let value_index = try!(reader.read_u16::<BigEndian>()) as usize;

        Ok(ConstantValueAttrInfo {
            value_index: value_index,
        })
    }

    pub fn value<'a>(&self, pool: &'a ConstantPool) -> Option<&'a ConstantPoolEntry> {
        pool.get(self.value_index)
    }
}

impl_print! {
    ConstantValueAttrInfo(self, printer, constant_pool: &ConstantPool) {
        let value = self.value(constant_pool).expect("Invalid value index");

        try!(printer.write_indent());
        try!(value.print(printer, constant_pool));
        try!(writeln!(printer, ""));
    }
}
