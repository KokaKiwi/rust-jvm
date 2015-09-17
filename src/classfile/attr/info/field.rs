use classfile::constant::{ConstantPool, ConstantPoolEntry};
use classfile::error::Result;
use podio::{ReadPodExt, BigEndian};
use std::io::{Read, Write};
use utils::print::{Print, Printer};

#[derive(Debug)]
pub struct ConstantValueAttrInfo {
    value_index: usize,
}

impl ConstantValueAttrInfo {
    pub fn read<R: Read>(reader: &mut R) -> Result<ConstantValueAttrInfo> {
        let value_index = try!(reader.read_u16::<BigEndian>()) as usize;

        Ok(ConstantValueAttrInfo {
            value_index: value_index,
        })
    }

    pub fn get_value<'a>(&self, pool: &'a ConstantPool) -> Option<&'a ConstantPoolEntry> {
        pool.get(self.value_index)
    }
}

impl<'a> Print<&'a ConstantPool> for ConstantValueAttrInfo {
    fn dump<W: Write>(&self, printer: &mut Printer<W, &'a ConstantPool>) -> ::std::io::Result<()> {
        let value = self.get_value(printer.context).unwrap();

        try!(printer.indent());
        try!(printer.print(value));

        Ok(())
    }
}
