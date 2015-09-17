mod error;

use podio::{ReadPodExt, BigEndian};
use std::io::{Read, Write};
pub use self::error::{Result, Error};
use utils::print::{Print, Printer};

#[allow(dead_code)]
mod tag {
    pub const CONSTANT_CLASS: u8              = 7;
    pub const CONSTANT_FIELDREF: u8           = 9;
    pub const CONSTANT_METHODREF: u8          = 10;
    pub const CONSTANT_INTERFACEMETHODREF: u8 = 11;
    pub const CONSTANT_STRING: u8             = 8;
    pub const CONSTANT_INTEGER: u8            = 3;
    pub const CONSTANT_FLOAT: u8              = 4;
    pub const CONSTANT_LONG: u8               = 5;
    pub const CONSTANT_DOUBLE: u8             = 6;
    pub const CONSTANT_NAMEANDTYPE: u8        = 12;
    pub const CONSTANT_UTF8: u8               = 1;
    pub const CONSTANT_METHODHANDLE: u8       = 15;
    pub const CONSTANT_METHODTYPE: u8         = 16;
    pub const CONSTANT_INVOKEDYNAMIC: u8      = 18;
}

#[derive(Debug)]
pub struct ConstantPool {
    entries: Vec<ConstantPoolEntry>,
}

impl ConstantPool {
    pub fn read<R: Read>(reader: &mut R) -> Result<ConstantPool> {
        let entries_count = try!(reader.read_u16::<BigEndian>()) as usize;
        let mut entries = Vec::with_capacity(entries_count - 1);

        let mut index = 1;
        while index < entries_count {
            let constant_pool_entry = try!(ConstantPoolEntry::read(reader));

            match constant_pool_entry {
                ConstantPoolEntry::Long(_) | ConstantPoolEntry::Double(_) => {
                    index += 1;
                }
                _ => {}
            }

            entries.push(constant_pool_entry);
            index += 1;
        }

        Ok(ConstantPool {
            entries: entries,
        })
    }

    pub fn get(&self, index: usize) -> Option<&ConstantPoolEntry> {
        // Indexes starts at 1 in Java classfiles...
        self.entries.get(index - 1)
    }

    pub fn get_str(&self, index: usize) -> Option<&str> {
        self.get(index).and_then(|entry| match entry {
            &ConstantPoolEntry::Utf8(ref info) => Some(info.get_value()),
            _ => None,
        })
    }

    pub fn get_class_info(&self, index: usize) -> Option<&ConstantClassInfo> {
        self.get(index).and_then(|entry| match entry {
            &ConstantPoolEntry::Class(ref info) => Some(info),
            _ => None,
        })
    }
}

impl Print for ConstantPool {
    fn dump<W: Write>(&self, printer: &mut Printer<W>) -> ::std::io::Result<()> {
        for (index, entry) in self.entries.iter().enumerate() {
            try!(printer.indent());
            try!(write!(printer, "{:3}: ", index));
            try!(printer.sub_context(self).print(entry));
            try!(writeln!(printer, ""));
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ConstantPoolEntry {
    Class(ConstantClassInfo),
    FieldRef(ConstantFieldRefInfo),
    MethodRef(ConstantMethodRefInfo),
    InterfaceMethodRef(ConstantInterfaceMethodRefInfo),
    String(ConstantStringInfo),
    Integer(ConstantIntegerInfo),
    Float(ConstantFloatInfo),
    Long(ConstantLongInfo),
    Double(ConstantDoubleInfo),
    NameAndType(ConstantNameAndTypeInfo),
    Utf8(ConstantUtf8Info),
    MethodHandle(ConstantMethodHandleInfo),
    MethodType(ConstantMethodTypeInfo),
    InvokedDynamic(ConstantInvokedDynamicInfo),
}

impl ConstantPoolEntry {
    pub fn read<R: Read>(reader: &mut R) -> Result<ConstantPoolEntry> {
        let tag = try!(reader.read_u8());

        match tag {
            tag::CONSTANT_CLASS => ConstantClassInfo::read(reader).map(ConstantPoolEntry::Class),
            tag::CONSTANT_FIELDREF => ConstantFieldRefInfo::read(reader).map(ConstantPoolEntry::FieldRef),
            tag::CONSTANT_METHODREF => ConstantMethodRefInfo::read(reader).map(ConstantPoolEntry::MethodRef),
            tag::CONSTANT_INTERFACEMETHODREF => ConstantInterfaceMethodRefInfo::read(reader).map(ConstantPoolEntry::InterfaceMethodRef),
            tag::CONSTANT_STRING => ConstantStringInfo::read(reader).map(ConstantPoolEntry::String),
            tag::CONSTANT_INTEGER => ConstantIntegerInfo::read(reader).map(ConstantPoolEntry::Integer),
            tag::CONSTANT_FLOAT => ConstantFloatInfo::read(reader).map(ConstantPoolEntry::Float),
            tag::CONSTANT_LONG => ConstantLongInfo::read(reader).map(ConstantPoolEntry::Long),
            tag::CONSTANT_DOUBLE => ConstantDoubleInfo::read(reader).map(ConstantPoolEntry::Double),
            tag::CONSTANT_NAMEANDTYPE => ConstantNameAndTypeInfo::read(reader).map(ConstantPoolEntry::NameAndType),
            tag::CONSTANT_UTF8 => ConstantUtf8Info::read(reader).map(ConstantPoolEntry::Utf8),
            tag::CONSTANT_METHODHANDLE => ConstantMethodHandleInfo::read(reader).map(ConstantPoolEntry::MethodHandle),
            tag::CONSTANT_METHODTYPE => ConstantMethodTypeInfo::read(reader).map(ConstantPoolEntry::MethodType),
            tag::CONSTANT_INVOKEDYNAMIC => ConstantInvokedDynamicInfo::read(reader).map(ConstantPoolEntry::InvokedDynamic),
            _ => Err(Error::BadTagValue(tag)),
        }
    }
}

impl<'a> Print<&'a ConstantPool> for ConstantPoolEntry {
    fn dump<W: Write>(&self, printer: &mut Printer<W, &'a ConstantPool>) -> ::std::io::Result<()> {
        match *self {
            ConstantPoolEntry::Class(ref info) => info.dump(printer),
            ConstantPoolEntry::FieldRef(ref info) => info.dump(printer),
            ConstantPoolEntry::MethodRef(ref info) => info.dump(printer),
            ConstantPoolEntry::InterfaceMethodRef(ref info) => info.dump(printer),
            ConstantPoolEntry::String(ref info) => info.dump(printer),
            ConstantPoolEntry::Integer(ref info) => info.dump(printer),
            ConstantPoolEntry::Float(ref info) => info.dump(printer),
            ConstantPoolEntry::Long(ref info) => info.dump(printer),
            ConstantPoolEntry::Double(ref info) => info.dump(printer),
            ConstantPoolEntry::NameAndType(ref info) => info.dump(printer),
            ConstantPoolEntry::Utf8(ref info) => info.dump(printer),
            ConstantPoolEntry::MethodHandle(ref info) => info.dump(printer),
            ConstantPoolEntry::MethodType(ref info) => info.dump(printer),
            ConstantPoolEntry::InvokedDynamic(ref info) => info.dump(printer),
        }
    }
}

#[derive(Debug)]
pub struct ConstantClassInfo {
    name_index: usize,
}

impl ConstantClassInfo {
    pub fn read<R: Read>(reader: &mut R) -> Result<ConstantClassInfo> {
        // Read name index
        let name_index = try!(reader.read_u16::<BigEndian>());

        Ok(ConstantClassInfo {
            name_index: name_index as usize,
        })
    }

    pub fn get_name<'a>(&self, pool: &'a ConstantPool) -> Option<&'a str> {
        pool.get_str(self.name_index)
    }
}

impl<'a> Print<&'a ConstantPool> for ConstantClassInfo {
    fn dump<W: Write>(&self, printer: &mut Printer<W, &'a ConstantPool>) -> ::std::io::Result<()> {
        let name = self.get_name(printer.context).unwrap();
        write!(printer, "Class `{}`", name)
    }
}

#[derive(Debug)]
pub struct ConstantFieldRefInfo {
    class_index: usize,
    name_and_type_index: usize,
}

impl ConstantFieldRefInfo {
    pub fn read<R: Read>(reader: &mut R) -> Result<ConstantFieldRefInfo> {
        // Read indexes
        let class_index = try!(reader.read_u16::<BigEndian>());
        let name_and_type_index = try!(reader.read_u16::<BigEndian>());

        Ok(ConstantFieldRefInfo {
            class_index: class_index as usize,
            name_and_type_index: name_and_type_index as usize,
        })
    }

    pub fn get_class<'a>(&self, pool: &'a ConstantPool) -> Option<&'a ConstantClassInfo> {
        pool.get_class_info(self.class_index)
    }

    pub fn get_name_and_type<'a>(&self, pool: &'a ConstantPool) -> Option<&'a ConstantNameAndTypeInfo> {
        pool.get(self.name_and_type_index).and_then(|entry| match *entry {
            ConstantPoolEntry::NameAndType(ref info) => Some(info),
            _ => None,
        })
    }
}

impl<'a> Print<&'a ConstantPool> for ConstantFieldRefInfo {
    fn dump<W: Write>(&self, printer: &mut Printer<W, &'a ConstantPool>) -> ::std::io::Result<()> {
        let class = self.get_class(printer.context).and_then(|class| class.get_name(printer.context)).unwrap();
        let name_and_type = self.get_name_and_type(printer.context).unwrap();
        let name = name_and_type.get_name(printer.context).unwrap();
        let desc = name_and_type.get_descriptor(printer.context).unwrap();

        write!(printer, "FieldRef `{}/{}` `{}`", class, name, desc)
    }
}

#[derive(Debug)]
pub struct ConstantMethodRefInfo {
    class_index: usize,
    name_and_type_index: usize,
}

impl ConstantMethodRefInfo {
    pub fn read<R: Read>(reader: &mut R) -> Result<ConstantMethodRefInfo> {
        // Read indexes
        let class_index = try!(reader.read_u16::<BigEndian>());
        let name_and_type_index = try!(reader.read_u16::<BigEndian>());

        Ok(ConstantMethodRefInfo {
            class_index: class_index as usize,
            name_and_type_index: name_and_type_index as usize,
        })
    }

    pub fn get_class<'a>(&self, pool: &'a ConstantPool) -> Option<&'a ConstantClassInfo> {
        pool.get_class_info(self.class_index)
    }

    pub fn get_name_and_type<'a>(&self, pool: &'a ConstantPool) -> Option<&'a ConstantNameAndTypeInfo> {
        pool.get(self.name_and_type_index).and_then(|entry| match *entry {
            ConstantPoolEntry::NameAndType(ref info) => Some(info),
            _ => None,
        })
    }
}

impl<'a> Print<&'a ConstantPool> for ConstantMethodRefInfo {
    fn dump<W: Write>(&self, printer: &mut Printer<W, &'a ConstantPool>) -> ::std::io::Result<()> {
        let class = self.get_class(printer.context).and_then(|class| class.get_name(printer.context)).unwrap();
        let name_and_type = self.get_name_and_type(printer.context).unwrap();
        let name = name_and_type.get_name(printer.context).unwrap();
        let desc = name_and_type.get_descriptor(printer.context).unwrap();

        write!(printer, "MethodRef `{}/{}` `{}`", class, name, desc)
    }
}

#[derive(Debug)]
pub struct ConstantInterfaceMethodRefInfo {
    class_index: usize,
    name_and_type_index: usize,
}

impl ConstantInterfaceMethodRefInfo {
    pub fn read<R: Read>(reader: &mut R) -> Result<ConstantInterfaceMethodRefInfo> {
        // Read indexes
        let class_index = try!(reader.read_u16::<BigEndian>());
        let name_and_type_index = try!(reader.read_u16::<BigEndian>());

        Ok(ConstantInterfaceMethodRefInfo {
            class_index: class_index as usize,
            name_and_type_index: name_and_type_index as usize,
        })
    }

    pub fn get_class<'a>(&self, pool: &'a ConstantPool) -> Option<&'a ConstantClassInfo> {
        pool.get_class_info(self.class_index)
    }

    pub fn get_name_and_type<'a>(&self, pool: &'a ConstantPool) -> Option<&'a ConstantNameAndTypeInfo> {
        pool.get(self.name_and_type_index).and_then(|entry| match *entry {
            ConstantPoolEntry::NameAndType(ref info) => Some(info),
            _ => None,
        })
    }
}

impl<'a> Print<&'a ConstantPool> for ConstantInterfaceMethodRefInfo {
    fn dump<W: Write>(&self, printer: &mut Printer<W, &'a ConstantPool>) -> ::std::io::Result<()> {
        let class = self.get_class(printer.context).and_then(|class| class.get_name(printer.context)).unwrap();
        let name_and_type = self.get_name_and_type(printer.context).unwrap();
        let name = name_and_type.get_name(printer.context).unwrap();
        let desc = name_and_type.get_descriptor(printer.context).unwrap();

        write!(printer, "InterfaceMethodRef `{}/{}` `{}`", class, name, desc)
    }
}

#[derive(Debug)]
pub struct ConstantStringInfo {
    string_index: usize,
}

impl ConstantStringInfo {
    pub fn read<R: Read>(reader: &mut R) -> Result<ConstantStringInfo> {
        let string_index = try!(reader.read_u16::<BigEndian>());

        Ok(ConstantStringInfo {
            string_index: string_index as usize,
        })
    }

    pub fn get_string<'a>(&self, pool: &'a ConstantPool) -> Option<&'a ConstantUtf8Info> {
        pool.get(self.string_index).and_then(|entry| match *entry {
            ConstantPoolEntry::Utf8(ref info) => Some(info),
            _ => None,
        })
    }

    pub fn get_value<'a>(&self, pool: &'a ConstantPool) -> Option<&'a str> {
        self.get_string(pool).map(ConstantUtf8Info::get_value)
    }
}

impl<'a> Print<&'a ConstantPool> for ConstantStringInfo {
    fn dump<W: Write>(&self, printer: &mut Printer<W, &'a ConstantPool>) -> ::std::io::Result<()> {
        let value = self.get_value(printer.context).unwrap();

        write!(printer, "String '{}'", value)
    }
}

#[derive(Debug)]
pub struct ConstantIntegerInfo {
    value: i32,
}

impl ConstantIntegerInfo {
    pub fn read<R: Read>(reader: &mut R) -> Result<ConstantIntegerInfo> {
        let value = try!(reader.read_i32::<BigEndian>());

        Ok(ConstantIntegerInfo {
            value: value,
        })
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

impl<'a> Print<&'a ConstantPool> for ConstantIntegerInfo {
    fn dump<W: Write>(&self, printer: &mut Printer<W, &'a ConstantPool>) -> ::std::io::Result<()> {
        write!(printer, "Integer `{}`", self.get_value())
    }
}

#[derive(Debug)]
pub struct ConstantFloatInfo {
    value: f32,
}

impl ConstantFloatInfo {
    pub fn read<R: Read>(reader: &mut R) -> Result<ConstantFloatInfo> {
        let value = try!(reader.read_f32::<BigEndian>());

        Ok(ConstantFloatInfo {
            value: value,
        })
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }
}

impl<'a> Print<&'a ConstantPool> for ConstantFloatInfo {
    fn dump<W: Write>(&self, printer: &mut Printer<W, &'a ConstantPool>) -> ::std::io::Result<()> {
        write!(printer, "Float `{}`", self.get_value())
    }
}

#[derive(Debug)]
pub struct ConstantLongInfo {
    value: i64,
}

impl ConstantLongInfo {
    pub fn read<R: Read>(reader: &mut R) -> Result<ConstantLongInfo> {
        let value = try!(reader.read_i64::<BigEndian>());

        Ok(ConstantLongInfo {
            value: value,
        })
    }

    pub fn get_value(&self) -> i64 {
        self.value
    }
}

impl<'a> Print<&'a ConstantPool> for ConstantLongInfo {
    fn dump<W: Write>(&self, printer: &mut Printer<W, &'a ConstantPool>) -> ::std::io::Result<()> {
        write!(printer, "Long `{}`", self.get_value())
    }
}

#[derive(Debug)]
pub struct ConstantDoubleInfo {
    value: f64,
}

impl ConstantDoubleInfo {
    pub fn read<R: Read>(reader: &mut R) -> Result<ConstantDoubleInfo> {
        let value = try!(reader.read_f64::<BigEndian>());

        Ok(ConstantDoubleInfo {
            value: value,
        })
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}

impl<'a> Print<&'a ConstantPool> for ConstantDoubleInfo {
    fn dump<W: Write>(&self, printer: &mut Printer<W, &'a ConstantPool>) -> ::std::io::Result<()> {
        write!(printer, "Double `{}`", self.get_value())
    }
}

#[derive(Debug)]
pub struct ConstantNameAndTypeInfo {
    name_index: usize,
    descriptor_index: usize,
}

impl ConstantNameAndTypeInfo {
    pub fn read<R: Read>(reader: &mut R) -> Result<ConstantNameAndTypeInfo> {
        // Read indexes
        let name_index = try!(reader.read_u16::<BigEndian>());
        let descriptor_index = try!(reader.read_u16::<BigEndian>());

        Ok(ConstantNameAndTypeInfo {
            name_index: name_index as usize,
            descriptor_index: descriptor_index as usize,
        })
    }

    pub fn get_name<'a>(&self, pool: &'a ConstantPool) -> Option<&'a str> {
        pool.get_str(self.name_index)
    }

    pub fn get_descriptor<'a>(&self, pool: &'a ConstantPool) -> Option<&'a str> {
        pool.get_str(self.descriptor_index)
    }
}

impl<'a> Print<&'a ConstantPool> for ConstantNameAndTypeInfo {
    fn dump<W: Write>(&self, printer: &mut Printer<W, &'a ConstantPool>) -> ::std::io::Result<()> {
        let name = self.get_name(printer.context).unwrap();
        let desc = self.get_descriptor(printer.context).unwrap();

        write!(printer, "NameAndType `{}`: `{}`", name, desc)
    }
}

#[derive(Debug)]
pub struct ConstantUtf8Info {
    value: String,
}

impl ConstantUtf8Info {
    pub fn read<R: Read>(reader: &mut R) -> Result<ConstantUtf8Info> {
        use utils::io::ReadExt;

        let length = try!(reader.read_u16::<BigEndian>()) as usize;
        let data = try!(reader.read_vec(length));
        let value = try!(String::from_utf8(data));

        Ok(ConstantUtf8Info {
            value: value,
        })
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}

impl<'a> Print<&'a ConstantPool> for ConstantUtf8Info {
    fn dump<W: Write>(&self, printer: &mut Printer<W, &'a ConstantPool>) -> ::std::io::Result<()> {
        write!(printer, "Utf8 `{}`", self.get_value())
    }
}

#[derive(Debug)]
pub struct ConstantMethodHandleInfo {
    ref_kind: u8, // TODO: Change this to an enum.
    ref_index: usize,
}

impl ConstantMethodHandleInfo {
    pub fn read<R: Read>(reader: &mut R) -> Result<ConstantMethodHandleInfo> {
        let ref_kind = try!(reader.read_u8());
        let ref_index = try!(reader.read_u16::<BigEndian>());

        Ok(ConstantMethodHandleInfo {
            ref_kind: ref_kind,
            ref_index: ref_index as usize,
        })
    }

    pub fn get_ref_kind(&self) -> u8 {
        self.ref_kind
    }

    pub fn get_ref<'a>(&self, pool: &'a ConstantPool) -> Option<&'a ConstantPoolEntry> {
        pool.get(self.ref_index)
    }
}

impl<'a> Print<&'a ConstantPool> for ConstantMethodHandleInfo {
    fn dump<W: Write>(&self, printer: &mut Printer<W, &'a ConstantPool>) -> ::std::io::Result<()> {
        write!(printer, "MethodHandle")
    }
}

#[derive(Debug)]
pub struct ConstantMethodTypeInfo {
    desc_index: usize,
}

impl ConstantMethodTypeInfo {
    pub fn read<R: Read>(reader: &mut R) -> Result<ConstantMethodTypeInfo> {
        let desc_index = try!(reader.read_u16::<BigEndian>());

        Ok(ConstantMethodTypeInfo {
            desc_index: desc_index as usize,
        })
    }

    pub fn get_desc<'a>(&self, pool: &'a ConstantPool) -> Option<&'a str> {
        pool.get_str(self.desc_index)
    }
}

impl<'a> Print<&'a ConstantPool> for ConstantMethodTypeInfo {
    fn dump<W: Write>(&self, printer: &mut Printer<W, &'a ConstantPool>) -> ::std::io::Result<()> {
        let desc = self.get_desc(printer.context).unwrap();

        write!(printer, "MethodType `{}`", desc)
    }
}

#[derive(Debug)]
pub struct ConstantInvokedDynamicInfo {
    bootstrap_method_attr_index: usize,
    name_and_type_index: usize,
}

impl ConstantInvokedDynamicInfo {
    pub fn read<R: Read>(reader: &mut R) -> Result<ConstantInvokedDynamicInfo> {
        let bootstrap_method_attr_index = try!(reader.read_u16::<BigEndian>());
        let name_and_type_index = try!(reader.read_u16::<BigEndian>());

        Ok(ConstantInvokedDynamicInfo {
            bootstrap_method_attr_index: bootstrap_method_attr_index as usize,
            name_and_type_index: name_and_type_index as usize,
        })
    }

    pub fn get_bootstrap_method_attr<'a>(&self, pool: &'a ConstantPool) -> Option<&'a ConstantPoolEntry> {
        pool.get(self.bootstrap_method_attr_index)
    }

    pub fn get_name_and_type<'a>(&self, pool: &'a ConstantPool) -> Option<&'a ConstantNameAndTypeInfo> {
        pool.get(self.name_and_type_index).and_then(|entry| match *entry {
            ConstantPoolEntry::NameAndType(ref info) => Some(info),
            _ => None,
        })
    }
}

impl<'a> Print<&'a ConstantPool> for ConstantInvokedDynamicInfo {
    fn dump<W: Write>(&self, printer: &mut Printer<W, &'a ConstantPool>) -> ::std::io::Result<()> {
        let name_and_type = self.get_name_and_type(printer.context).unwrap();
        let name = name_and_type.get_name(printer.context).unwrap();
        let desc = name_and_type.get_descriptor(printer.context).unwrap();

        write!(printer, "InvokedDynamic `{}` `{}`", name, desc)
    }
}
