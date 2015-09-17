pub mod attr;
pub mod constant;
pub mod error;
pub mod field;
pub mod method;
pub mod version;

use podio::{ReadPodExt, BigEndian};
use self::attr::Attr;
use self::constant::ConstantPool;
pub use self::error::{Result, Error};
use self::field::FieldInfo;
use self::method::MethodInfo;
use self::version::Version;
use std::fmt;
use std::io::{Read, Write};
use utils::print::{Print, Printer};

const MAGIC_VALUE: u32 = 0xCAFEBABE;

#[derive(Debug)]
pub struct ClassFile {
    pub version: Version,
    pub constant_pool: ConstantPool,
    pub access_flags: AccessFlags,
    this_class: usize,
    super_class: usize,
    pub interfaces: Vec<usize>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attrs: Vec<Attr>,
}

impl ClassFile {
    pub fn read<R: Read>(reader: &mut R) -> Result<ClassFile> {
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
        let access_flags = match AccessFlags::from_bits(access_flags) {
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

    pub fn get_this_class(&self) -> Option<&constant::ConstantClassInfo> {
        self.constant_pool.get_class_info(self.this_class)
    }

    pub fn get_super_class(&self) -> Option<&constant::ConstantClassInfo> {
        self.constant_pool.get_class_info(self.super_class)
    }
}

impl Print for ClassFile {
    fn dump<W: Write>(&self, printer: &mut Printer<W>) -> ::std::io::Result<()> {
        try!(printer.indent());
        try!(writeln!(printer, "Version: {}", self.version));

        try!(printer.indent());
        try!(writeln!(printer, "Constants:"));
        try!(printer.sub().with_indent(4).print(&self.constant_pool));

        try!(printer.indent());
        try!(writeln!(printer, "Access flags: {}", self.access_flags));

        try!(printer.indent());
        try!(write!(printer, "This class: "));
        try!(printer.sub_context(&self.constant_pool).with_indent(4).print(self.get_this_class().unwrap()));
        try!(writeln!(printer, ""));

        try!(printer.indent());
        try!(write!(printer, "Super class: "));
        try!(printer.sub_context(&self.constant_pool).with_indent(4).print(self.get_super_class().unwrap()));
        try!(writeln!(printer, ""));

        try!(printer.indent());
        try!(writeln!(printer, "Interfaces:"));
        for info in self.interfaces.iter().map(|&index| self.constant_pool.get_class_info(index).unwrap()) {
            try!(printer.sub().with_indent(4).indent());
            try!(printer.sub_context(&self.constant_pool).print(info));
            try!(writeln!(printer, ""));
        }

        try!(printer.indent());
        try!(writeln!(printer, "Fields:"));
        for field in self.fields.iter() {
            try!(printer.sub_context(&self.constant_pool).with_indent(4).print(field));
        }

        try!(printer.indent());
        try!(writeln!(printer, "Methods:"));
        for method in self.methods.iter() {
            try!(printer.sub_context(&self.constant_pool).with_indent(4).print(method));
        }

        try!(printer.indent());
        try!(writeln!(printer, "Attributes:"));
        for attr in self.attrs.iter() {
            try!(printer.sub_context(&self.constant_pool).with_indent(4).print(attr));
        }

        Ok(())
    }
}

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

impl fmt::Display for AccessFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut flags = Vec::new();
        if self.contains(ACC_PUBLIC) { flags.push("ACC_PUBLIC"); }
        if self.contains(ACC_FINAL) { flags.push("ACC_FINAL"); }
        if self.contains(ACC_SUPER) { flags.push("ACC_SUPER"); }
        if self.contains(ACC_INTERFACE) { flags.push("ACC_INTERFACE"); }
        if self.contains(ACC_ABSTRACT) { flags.push("ACC_ABSTRACT"); }
        if self.contains(ACC_SYNTHETIC) { flags.push("ACC_SYNTHETIC"); }
        if self.contains(ACC_ANNOTATION) { flags.push("ACC_ANNOTATION"); }
        if self.contains(ACC_ENUM) { flags.push("ACC_ENUM"); }
        write!(f, "{}", flags.join(", "))
    }
}
