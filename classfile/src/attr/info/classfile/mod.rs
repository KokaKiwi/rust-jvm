use byteorder::{ReadBytesExt, BigEndian};
use constant::{ConstantPool, ConstantPoolEntry, ConstantClassInfo, ConstantNameAndTypeInfo};
use error::Result;
use std::io::Read;
pub use self::inner_classes::InnerClassesAttrInfo;

pub mod inner_classes;

#[derive(Debug)]
pub struct SourceFileAttrInfo {
    sourcefile_index: usize,
}

impl SourceFileAttrInfo {
    pub fn read<R: Read>(reader: &mut R, _constant_pool: &ConstantPool) -> Result<SourceFileAttrInfo> {
        let sourcefile_index = try!(reader.read_u16::<BigEndian>()) as usize;

        Ok(SourceFileAttrInfo {
            sourcefile_index: sourcefile_index,
        })
    }

    pub fn sourcefile<'a>(&self, constant_pool: &'a ConstantPool) -> Option<&'a str> {
        constant_pool.get_str(self.sourcefile_index)
    }
}

impl_print! {
    SourceFileAttrInfo(self, printer, constant_pool: &ConstantPool) {
        let sourcefile = self.sourcefile(constant_pool).expect("Invalid index");

        try!(printer.write_indent());
        try!(writeln!(printer, "Source file: {}", sourcefile));
    }
}

#[derive(Debug)]
pub struct EnclosingMethodAttrInfo {
    class_index: usize,
    method_index: usize,
}

impl EnclosingMethodAttrInfo {
    pub fn read<R: Read>(reader: &mut R, _constant_pool: &ConstantPool) -> Result<EnclosingMethodAttrInfo> {
        let class_index = try!(reader.read_u16::<BigEndian>()) as usize;
        let method_index = try!(reader.read_u16::<BigEndian>()) as usize;

        Ok(EnclosingMethodAttrInfo {
            class_index: class_index,
            method_index: method_index,
        })
    }

    pub fn class<'a>(&self, constant_pool: &'a ConstantPool) -> Option<&'a ConstantClassInfo> {
        constant_pool.get_class_info(self.class_index)
    }

    pub fn method<'a>(&self, constant_pool: &'a ConstantPool) -> Option<&'a ConstantNameAndTypeInfo> {
        if self.method_index != 0 {
            constant_pool.get(self.method_index).and_then(|entry| match entry {
                &ConstantPoolEntry::NameAndType(ref info) => Some(info),
                _ => None,
            })
        } else {
            None
        }
    }
}

impl_print! {
    EnclosingMethodAttrInfo(self, printer, constant_pool: &ConstantPool) {
        let class = self.class(constant_pool).expect("Invalid class index");

        try!(printer.write_indent());
        try!(class.print(printer, constant_pool));
        try!(writeln!(printer, ""));

        if let Some(method) = self.method(constant_pool) {
            try!(printer.write_indent());
            try!(method.print(printer, constant_pool));
            try!(writeln!(printer, ""));
        }
    }
}
