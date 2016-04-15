use attr::Attr;
use constant::{ConstantPool, ConstantClassInfo};
use error::Result;
use std::io::Read;

#[derive(Debug)]
pub struct CodeAttrInfo {
    pub max_stack: usize,
    pub max_locals: usize,
    pub code: Vec<u8>,
    pub exception_handlers: Vec<ExceptionHandler>,
    pub attrs: Vec<Attr>,
}

impl_read! {
    CodeAttrInfo(reader, constant_pool: &ConstantPool) -> Result<Self> = {
        use utils::io::ReadExt;

        // Read indexes
        let max_stack = try!(reader.read_u16::<BigEndian>()) as usize;
        let max_locals = try!(reader.read_u16::<BigEndian>()) as usize;

        // Read code
        let code_size = try!(reader.read_u32::<BigEndian>()) as usize;
        let code = try!(reader.read_vec(code_size));

        // Read exception table
        let exception_handlers_count = try!(reader.read_u16::<BigEndian>()) as usize;
        let mut exception_handlers = Vec::with_capacity(exception_handlers_count);
        for _ in 0..exception_handlers_count {
            let exception_handler = try!(ExceptionHandler::read(reader));
            exception_handlers.push(exception_handler);
        }

        // Read attributes
        let attrs_count = try!(reader.read_u16::<BigEndian>()) as usize;
        let mut attrs = Vec::with_capacity(attrs_count);
        for _ in 0..attrs_count {
            let attr = try!(Attr::read(reader, constant_pool));
            attrs.push(attr);
        }

        Ok(CodeAttrInfo {
            max_stack: max_stack,
            max_locals: max_locals,
            code: code,
            exception_handlers: exception_handlers,
            attrs: attrs,
        })
    }
}

impl_print! {
    CodeAttrInfo(self, printer, constant_pool: &ConstantPool) {
        try!(printer.write_indent());
        try!(writeln!(printer, "Max stack: {}", self.max_stack));

        try!(printer.write_indent());
        try!(writeln!(printer, "Max locals: {}", self.max_locals));

        try!(printer.write_indent());
        try!(writeln!(printer, "Code: [ {} bytes ]", self.code.len()));

        try!(printer.write_indent());
        try!(writeln!(printer, "Exception handlers:"));
        for handler in self.exception_handlers.iter() {
            try!(handler.print(&mut printer.sub_indent(1), constant_pool));
        }

        try!(printer.write_indent());
        try!(writeln!(printer, "Attrs:"));
        for attr in self.attrs.iter() {
            try!(attr.print(&mut printer.sub_indent(1).by_ref(), constant_pool));
        }
    }
}

#[derive(Debug)]
pub struct ExceptionHandler {
    pub start_pc: usize,
    pub end_pc: usize,
    pub handler_pc: usize,
    catch_type: usize,
}

impl ExceptionHandler {
    pub fn catch_type<'a>(&self, pool: &'a ConstantPool) -> Option<&'a ConstantClassInfo> {
        if self.catch_type != 0 {
            pool.get_class_info(self.catch_type)
        } else {
            None
        }
    }
}

impl_read! {
    ExceptionHandler(reader) -> Result<Self> = {
        let start_pc = try!(reader.read_u16::<BigEndian>()) as usize;
        let end_pc = try!(reader.read_u16::<BigEndian>()) as usize;
        let handler_pc = try!(reader.read_u16::<BigEndian>()) as usize;
        let catch_type = try!(reader.read_u16::<BigEndian>()) as usize;

        Ok(ExceptionHandler {
            start_pc: start_pc,
            end_pc: end_pc,
            handler_pc: handler_pc,
            catch_type: catch_type,
        })
    }
}

impl_print! {
    ExceptionHandler(self, printer, constant_pool: &ConstantPool) {
        try!(printer.write_indent());
        try!(write!(printer, "Exception handler [{:#x}:{:#x}] @ {:#x}", self.start_pc, self.end_pc, self.handler_pc));

        if let Some(catch_type) = self.catch_type(constant_pool) {
            try!(write!(printer, " -> "));
            try!(catch_type.print(printer, constant_pool));
        }

        try!(writeln!(printer, ""));
    }
}
