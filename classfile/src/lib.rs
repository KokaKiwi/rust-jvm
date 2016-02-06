#[macro_use] extern crate bitflags;
extern crate byteorder;
#[macro_use] extern crate log;
#[macro_use] extern crate quick_error;

#[macro_use] mod utils;
pub mod attr;
pub mod constant;
pub mod error;
pub mod field;
pub mod method;
pub mod version;

use byteorder::{ReadBytesExt, BigEndian};
use self::attr::Attr;
use self::constant::ConstantPool;
pub use self::error::{Result, Error};
use self::field::FieldInfo;
use self::method::MethodInfo;
use self::version::Version;
use std::io;
use utils::print::Printer;

const MAGIC_VALUE: u32 = 0xCAFEBABE;

#[derive(Debug)]
pub struct ClassFile {
    pub version: Version,
    pub constant_pool: ConstantPool,
    pub access_flags: flags::AccessFlags,
    this_class: usize,
    super_class: usize,
    interfaces: Vec<usize>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attrs: Vec<Attr>,
}

impl ClassFile {
    pub fn read<R: io::Read>(reader: &mut R) -> Result<ClassFile> {
        // Read magic value
        let magic = try!(reader.read_u32::<BigEndian>());
        if magic != MAGIC_VALUE {
            return Err(Error::BadMagicValue(magic));
        }

        // Read version
        let minor = try!(reader.read_u16::<BigEndian>());
        let major = try!(reader.read_u16::<BigEndian>());

        // Read constant pool
        let constant_pool = try!(ConstantPool::read(reader));

        // Read access flags
        let access_flags = try!(reader.read_u16::<BigEndian>());
        let access_flags = match flags::AccessFlags::from_bits(access_flags) {
            Some(flags) => flags,
            None => return Err(Error::BadAccessFlags(access_flags)),
        };

        // Read indexes
        let this_class = try!(reader.read_u16::<BigEndian>()) as usize;
        let super_class = try!(reader.read_u16::<BigEndian>()) as usize;

        // Read interfaces
        let interfaces_count = try!(reader.read_u16::<BigEndian>()) as usize;
        let mut interfaces = Vec::with_capacity(interfaces_count);
        for _ in 0..interfaces_count {
            let interface_index = try!(reader.read_u16::<BigEndian>()) as usize;
            interfaces.push(interface_index);
        }

        // Read fields
        let fields_count = try!(reader.read_u16::<BigEndian>()) as usize;
        let mut fields = Vec::with_capacity(fields_count);
        for _ in 0..fields_count {
            let field = try!(FieldInfo::read(reader, &constant_pool));
            fields.push(field);
        }

        // Read methods
        let methods_count = try!(reader.read_u16::<BigEndian>()) as usize;
        let mut methods = Vec::with_capacity(methods_count);
        for _ in 0..methods_count {
            let method = try!(MethodInfo::read(reader, &constant_pool));
            methods.push(method);
        }

        // Read attributes
        let attrs_count = try!(reader.read_u16::<BigEndian>()) as usize;
        let mut attrs = Vec::with_capacity(attrs_count);
        for _ in 0..attrs_count {
            let attr = try!(Attr::read(reader, &constant_pool));
            attrs.push(attr);
        }

        Ok(ClassFile {
            version: Version::new(major, minor),
            constant_pool: constant_pool,
            access_flags: access_flags,
            this_class: this_class,
            super_class: super_class,
            interfaces: interfaces,
            fields: fields,
            methods: methods,
            attrs: attrs,
        })
    }

    pub fn this_class(&self) -> Option<&constant::ConstantClassInfo> {
        self.constant_pool.get_class_info(self.this_class)
    }

    pub fn super_class(&self) -> Option<&constant::ConstantClassInfo> {
        self.constant_pool.get_class_info(self.super_class)
    }

    pub fn interfaces<'a>(&'a self) -> Interfaces<'a> {
        Interfaces::new(self)
    }

    pub fn dump(&self) {
        let mut printer = Printer::default();
        self.print(&mut printer).unwrap();
    }
}

impl_print! {
    ClassFile(self, printer) {
        try!(printer.write_indent());
        try!(writeln!(printer, "Version: {}", self.version));

        try!(printer.write_indent());
        try!(writeln!(printer, "Access flags: {:?}", self.access_flags));

        let this_class = self.this_class().expect("Invalid class index");
        try!(printer.write_indent());
        try!(write!(printer, "This class: "));
        try!(this_class.print(printer, &self.constant_pool));
        try!(writeln!(printer, ""));

        let super_class = self.super_class().expect("Invalid class index");
        try!(printer.write_indent());
        try!(write!(printer, "Super class: "));
        try!(super_class.print(printer, &self.constant_pool));
        try!(writeln!(printer, ""));

        try!(printer.write_indent());
        try!(writeln!(printer, "Constants:"));
        try!(self.constant_pool.print(&mut printer.sub_indent(1)));

        try!(printer.write_indent());
        try!(writeln!(printer, "Interfaces:"));
        for iface in self.interfaces() {
            if let Some(iface) = iface {
                try!(iface.print(&mut printer.sub_indent(1), &self.constant_pool));
            }
        }

        try!(printer.write_indent());
        try!(writeln!(printer, "Fields:"));
        for field in self.fields.iter() {
            try!(field.print(&mut printer.sub_indent(1), &self.constant_pool));
        }

        try!(printer.write_indent());
        try!(writeln!(printer, "Methods:"));
        for method in self.methods.iter() {
            try!(method.print(&mut printer.sub_indent(1), &self.constant_pool));
        }

        try!(printer.write_indent());
        try!(writeln!(printer, "Attrs:"));
        for attr in self.attrs.iter() {
            try!(attr.print(&mut printer.sub_indent(1), &self.constant_pool));
        }
    }
}

pub struct Interfaces<'a> {
    iter: ::std::slice::Iter<'a, usize>,
    constant_pool: &'a ConstantPool,
}

impl<'a> Interfaces<'a> {
    fn new(cf: &'a ClassFile) -> Interfaces<'a> {
        Interfaces {
            iter: cf.interfaces.iter(),
            constant_pool: &cf.constant_pool,
        }
    }
}

impl<'a> Iterator for Interfaces<'a> {
    type Item = Option<&'a constant::ConstantClassInfo>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|&index| self.constant_pool.get_class_info(index))
    }
}

pub mod flags {
    bitflags! {
        flags AccessFlags: u16 {
            #[doc = "Declared public; may be accessed from outside its package."]
            const ACC_PUBLIC      = 0x0001,
            #[doc = "Declared final; no subclasses allowed."]
            const ACC_FINAL       = 0x0010,
            #[doc = "Treat superclass methods specially when invoked by the invokespecial instruction."]
            const ACC_SUPER       = 0x0020,
            #[doc = "Is an interface, not a class."]
            const ACC_INTERFACE   = 0x0200,
            #[doc = "Declared abstract; must not be instantiated."]
            const ACC_ABSTRACT    = 0x0400,
            #[doc = "Declared synthetic; not present in the source code."]
            const ACC_SYNTHETIC   = 0x1000,
            #[doc = "Declared as an annotation type."]
            const ACC_ANNOTATION  = 0x2000,
            #[doc = "Declared as an enum type."]
            const ACC_ENUM        = 0x4000,
        }
    }
}
