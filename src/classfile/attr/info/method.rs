use podio::{ReadPodExt, BigEndian};
use classfile::constant::ConstantPool;
use classfile::error::Result;
use std::io::{Read, Write};
use utils::print::{Print, Printer};

#[derive(Debug)]
pub struct CodeAttrInfo {
    pub max_stack: usize,
    pub max_locals: usize,
    pub code: Vec<u8>,
}

impl CodeAttrInfo {
    pub fn read<R: Read>(reader: &mut R) -> Result<CodeAttrInfo> {
        use utils::io::ReadExt;

        // Read indexes
        let max_stack = try!(reader.read_u16::<BigEndian>()) as usize;
        let max_locals = try!(reader.read_u16::<BigEndian>()) as usize;

        // Read code
        let code_size = try!(reader.read_u32::<BigEndian>()) as usize;
        let code = try!(reader.read_vec(code_size));

        Ok(CodeAttrInfo {
            max_stack: max_stack,
            max_locals: max_locals,
            code: code,
        })
    }
}

impl<'a> Print<&'a ConstantPool> for CodeAttrInfo {
    fn dump<W: Write>(&self, printer: &mut Printer<W, &'a ConstantPool>) -> ::std::io::Result<()> {
        try!(printer.indent());
        try!(writeln!(printer, "Max stack: {}", self.max_stack));

        try!(printer.indent());
        try!(writeln!(printer, "Max locals: {}", self.max_locals));

        try!(printer.indent());
        try!(writeln!(printer, "Code: [ {} bytes... ]", self.code.len()));

        Ok(())
    }
}
