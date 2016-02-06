use byteorder::{ReadBytesExt, BigEndian};
use constant::ConstantPool;
use error::Result;
use std::slice::Iter;
use std::io::Read;

#[derive(Debug)]
pub struct LocalVariableTableAttrInfo {
    entries: Vec<LocalVariable>,
}

impl LocalVariableTableAttrInfo {
    pub fn read<R: Read>(reader: &mut R, _constant_pool: &ConstantPool) -> Result<LocalVariableTableAttrInfo> {
        // Read entries
        let entries_count = try!(reader.read_u16::<BigEndian>()) as usize;
        let mut entries = Vec::with_capacity(entries_count);
        for _ in 0..entries_count {
            let entry = try!(LocalVariable::read(reader));
            entries.push(entry);
        }

        Ok(LocalVariableTableAttrInfo {
            entries: entries,
        })
    }

    pub fn entries<'a>(&'a self) -> Iter<'a, LocalVariable> {
        self.entries.iter()
    }
}

impl_print! {
    LocalVariableTableAttrInfo(self, printer, constant_pool: &ConstantPool) {
        for entry in self.entries() {
            try!(entry.print(printer, constant_pool));
        }
    }
}

#[derive(Debug)]
pub struct LocalVariable {
    pub start_pc: usize,
    pub length: usize,
    name_index: usize,
    desc_index: usize,
    pub index: usize,
}

impl LocalVariable {
    pub fn read<R: Read>(reader: &mut R) -> Result<LocalVariable> {
        let start_pc = try!(reader.read_u16::<BigEndian>()) as usize;
        let length = try!(reader.read_u16::<BigEndian>()) as usize;
        let name_index = try!(reader.read_u16::<BigEndian>()) as usize;
        let desc_index = try!(reader.read_u16::<BigEndian>()) as usize;
        let index = try!(reader.read_u16::<BigEndian>()) as usize;

        Ok(LocalVariable {
            start_pc: start_pc,
            length: length,
            name_index: name_index,
            desc_index: desc_index,
            index: index,
        })
    }

    pub fn name<'a>(&self, constant_pool: &'a ConstantPool) -> Option<&'a str> {
        constant_pool.get_str(self.name_index)
    }

    pub fn desc<'a>(&self, constant_pool: &'a ConstantPool) -> Option<&'a str> {
        constant_pool.get_str(self.desc_index)
    }
}

impl_print! {
    LocalVariable(self, printer, constant_pool: &ConstantPool) {
        let name = self.name(constant_pool).expect("Invalid name index");
        let desc = self.desc(constant_pool).expect("Invalid desc index");

        let start = self.start_pc;
        let end = start + self.length;

        try!(printer.write_indent());
        try!(writeln!(printer, "Local variable `{}` [{}] @ {:#x}:", name, desc, self.index));

        {
            let mut printer = printer.sub_indent(1);

            try!(printer.write_indent());
            try!(writeln!(printer, "Location: [{:#x}:{:#x}]", start, end));
        }
    }
}
