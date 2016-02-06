use byteorder::{ReadBytesExt, BigEndian};
use constant::{ConstantPool, ConstantClassInfo};
use error::{Error, Result};
use std::io::Read;

#[derive(Debug)]
pub struct InnerClassesAttrInfo {
    pub classes: Vec<Class>,
}

impl InnerClassesAttrInfo {
    pub fn read<R: Read>(reader: &mut R, _pool: &ConstantPool) -> Result<Self> {
        let classes_count = try!(reader.read_u16::<BigEndian>()) as usize;
        let mut classes = Vec::with_capacity(classes_count);
        for _ in 0..classes_count {
            let class = try!(Class::read(reader));
            classes.push(class);
        }

        Ok(InnerClassesAttrInfo {
            classes: classes,
        })
    }
}

impl_print! {
    InnerClassesAttrInfo(self, printer, constant_pool: &ConstantPool) {
        for class in self.classes.iter() {
            try!(printer.write_indent());
            try!(writeln!(printer, "Class:"));

            try!(class.print(&mut printer.sub_indent(1), constant_pool));
        }
    }
}

#[derive(Debug)]
pub struct Class {
    inner_class_info_index: usize,
    outer_class_info_index: usize,
    inner_name_index: usize,
    pub inner_class_access_flags: flags::AccessFlags,
}

impl Class {
    pub fn read<R: Read>(reader: &mut R) -> Result<Class> {
        let inner_class_info_index = try!(reader.read_u16::<BigEndian>()) as usize;
        let outer_class_info_index = try!(reader.read_u16::<BigEndian>()) as usize;
        let inner_name_index = try!(reader.read_u16::<BigEndian>()) as usize;
        let inner_class_access_flags = try!(reader.read_u16::<BigEndian>());
        let inner_class_access_flags = match flags::AccessFlags::from_bits(inner_class_access_flags) {
            Some(flags) => flags,
            None => return Err(Error::BadAccessFlags(inner_class_access_flags)),
        };

        Ok(Class {
            inner_class_info_index: inner_class_info_index,
            outer_class_info_index: outer_class_info_index,
            inner_name_index: inner_name_index,
            inner_class_access_flags: inner_class_access_flags,
        })
    }

    pub fn inner_class_info<'a>(&self, constant_pool: &'a ConstantPool) -> Option<&'a ConstantClassInfo> {
        constant_pool.get_class_info(self.inner_class_info_index)
    }

    pub fn outer_class_info<'a>(&self, constant_pool: &'a ConstantPool) -> Option<&'a ConstantClassInfo> {
        if self.outer_class_info_index != 0 {
            constant_pool.get_class_info(self.outer_class_info_index)
        } else {
            None
        }
    }

    pub fn inner_name<'a>(&self, constant_pool: &'a ConstantPool) -> Option<&'a str> {
        if self.inner_name_index != 0 {
            constant_pool.get_str(self.inner_name_index)
        } else {
            None
        }
    }
}

impl_print! {
    Class(self, printer, constant_pool: &ConstantPool) {
        let inner_class_info = self.inner_class_info(constant_pool).expect("Invalid index");

        try!(printer.write_indent());
        try!(write!(printer, "Inner class: "));
        try!(inner_class_info.print(printer, constant_pool));
        try!(writeln!(printer, ""));

        try!(printer.write_indent());
        try!(writeln!(printer, "Access flags: {:?}", self.inner_class_access_flags));

        if let Some(outer_class_info) = self.outer_class_info(constant_pool) {
            try!(printer.write_indent());
            try!(write!(printer, "Outer class: "));
            try!(outer_class_info.print(printer, constant_pool));
            try!(writeln!(printer, ""));
        }

        if let Some(inner_name) = self.inner_name(constant_pool) {
            try!(printer.write_indent());
            try!(writeln!(printer, "Inner name: {}", inner_name));
        }
    }
}

pub mod flags {
    bitflags! {
        flags AccessFlags: u16 {
            #[doc = "Marked or implicitly public in source."]
            const ACC_PUBLIC = 0x0001,
            #[doc = "Marked private in source."]
            const ACC_PRIVATE = 0x0002,
            #[doc = "Marked protected in source."]
            const ACC_PROTECTED = 0x0004,
            #[doc = "Marked or implicitly static in source."]
            const ACC_STATIC = 0x0008,
            #[doc = "Marked final in source."]
            const ACC_FINAL = 0x0010,
            #[doc = "Was an interface in source."]
            const ACC_INTERFACE = 0x0200,
            #[doc = "Marked or implicitly abstract in source."]
            const ACC_ABSTRACT = 0x0400,
            #[doc = "Declared synthetic; not present in the source code."]
            const ACC_SYNTHETIC = 0x1000,
            #[doc = "Declared as an annotation type."]
            const ACC_ANNOTATION = 0x2000,
            #[doc = "Declared as an enum type."]
            const ACC_ENUM = 0x4000,
        }
    }
}
