use byteorder::{ReadBytesExt, BigEndian};
use constant::{ConstantPool, ConstantClassInfo};
use error::Result;
use std::io::Read;

#[derive(Debug)]
pub struct ExceptionsAttrInfo {
    table: Vec<usize>,
}

impl ExceptionsAttrInfo {
    pub fn read<R: Read>(reader: &mut R, _pool: &ConstantPool) -> Result<ExceptionsAttrInfo> {
        let table_size = try!(reader.read_u16::<BigEndian>()) as usize;
        let mut table = Vec::with_capacity(table_size);
        for _ in 0..table_size {
            let index = try!(reader.read_u16::<BigEndian>()) as usize;
            table.push(index);
        }

        Ok(ExceptionsAttrInfo {
            table: table,
        })
    }

    pub fn table<'a, 'b>(&'a self, pool: &'b ConstantPool) -> ExceptionsTable<'a, 'b> {
        ExceptionsTable::new(self, pool)
    }
}

impl_print! {
    ExceptionsAttrInfo(self, printer, constant_pool: &ConstantPool) {
        for class in self.table(constant_pool).filter_map(|class| class) {
            try!(printer.write_indent());
            try!(class.print(printer, constant_pool));
            try!(writeln!(printer, ""));
        }
    }
}

pub struct ExceptionsTable<'a, 'b> {
    iter: ::std::slice::Iter<'a, usize>,
    constant_pool: &'b ConstantPool,
}

impl<'a, 'b> ExceptionsTable<'a, 'b> {
    fn new(info: &'a ExceptionsAttrInfo, constant_pool: &'b ConstantPool) -> Self {
        ExceptionsTable {
            iter: info.table.iter(),
            constant_pool: constant_pool,
        }
    }
}

impl<'a, 'b> Iterator for ExceptionsTable<'a, 'b> {
    type Item = Option<&'b ConstantClassInfo>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|&index| self.constant_pool.get_class_info(index))
    }
}
