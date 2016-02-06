use byteorder::{ReadBytesExt, BigEndian};
use constant::ConstantPool;
use error::Result;
use std::slice::Iter;
use std::io::Read;

#[derive(Debug)]
pub struct LineNumberTableAttrInfo {
    entries: Vec<LineNumber>,
}

impl LineNumberTableAttrInfo {
    pub fn read<R: Read>(reader: &mut R, _constant_pool: &ConstantPool) -> Result<LineNumberTableAttrInfo> {
        // Read entries
        let entries_count = try!(reader.read_u16::<BigEndian>()) as usize;
        let mut entries = Vec::with_capacity(entries_count);
        for _ in 0..entries_count {
            let entry = try!(LineNumber::read(reader));
            entries.push(entry);
        }

        Ok(LineNumberTableAttrInfo {
            entries: entries,
        })
    }

    pub fn entries<'a>(&'a self) -> Iter<'a, LineNumber> {
        self.entries.iter()
    }
}

impl_print! {
    LineNumberTableAttrInfo(self, printer) {
        for entry in self.entries() {
            try!(printer.write_indent());
            try!(entry.print(printer));
            try!(writeln!(printer, ""));
        }
    }
}

#[derive(Debug)]
pub struct LineNumber {
    pub start_pc: usize,
    pub line_number: usize,
}

impl LineNumber {
    pub fn read<R: Read>(reader: &mut R) -> Result<LineNumber> {
        let start_pc = try!(reader.read_u16::<BigEndian>()) as usize;
        let line_number = try!(reader.read_u16::<BigEndian>()) as usize;

        Ok(LineNumber {
            start_pc: start_pc,
            line_number: line_number,
        })
    }
}

impl_print! {
    LineNumber(self, printer) {
        try!(write!(printer, "{:#x} = Line {}", self.start_pc, self.line_number));
    }
}
