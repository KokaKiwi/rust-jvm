use constant::{ConstantPool, ConstantPoolEntry};
use error::{Result, Error};
use std::slice::Iter;

#[derive(Debug)]
pub struct RuntimeVisibleAnnotationsAttrInfo {
    annotations: Vec<Annotation>,
}

impl RuntimeVisibleAnnotationsAttrInfo {
    pub fn annotations<'a>(&'a self) -> Iter<'a, Annotation> {
        self.annotations.iter()
    }
}

impl_read! {
    RuntimeVisibleAnnotationsAttrInfo(reader, _constant_pool: &ConstantPool) -> Result<Self> = {
        let annotations_count = try!(reader.read_u16::<BigEndian>()) as usize;
        let mut annotations = Vec::with_capacity(annotations_count);
        for _ in 0..annotations_count {
            let annotation = try!(Annotation::read(reader));
            annotations.push(annotation);
        }

        Ok(RuntimeVisibleAnnotationsAttrInfo {
            annotations: annotations,
        })
    }
}

impl_print! {
    RuntimeVisibleAnnotationsAttrInfo(self, printer, constant_pool: &ConstantPool) {
        for annotation in self.annotations() {
            try!(printer.write_indent());
            try!(annotation.print(printer, constant_pool));
        }
    }
}

#[derive(Debug)]
pub struct Annotation {
    type_index: usize,
    element_values: Vec<NamedElementValue>,
}

impl Annotation {
    pub fn ty<'a>(&self, constant_pool: &'a ConstantPool) -> Option<&'a str> {
        constant_pool.get_str(self.type_index)
    }

    pub fn element_values<'a>(&'a self) -> Iter<'a, NamedElementValue> {
        self.element_values.iter()
    }
}

impl_read! {
    Annotation(reader) -> Result<Annotation> = {
        let type_index = try!(reader.read_u16::<BigEndian>()) as usize;

        // Read element values
        let element_values_count = try!(reader.read_u16::<BigEndian>()) as usize;
        let mut element_values = Vec::with_capacity(element_values_count);
        for _ in 0..element_values_count {
            let element_value = try!(NamedElementValue::read(reader));
            element_values.push(element_value);
        }

        Ok(Annotation {
            type_index: type_index,
            element_values: element_values,
        })
    }
}

impl_print! {
    Annotation(self, printer, constant_pool: &ConstantPool) {
        let ty = self.ty(constant_pool).expect("Invalid type index");

        try!(writeln!(printer, "Annotation [{}]:", ty));

        for element_value in self.element_values() {
            try!(element_value.print(&mut printer.sub_indent(1), constant_pool));
        }
    }
}

#[derive(Debug)]
pub struct NamedElementValue {
    name_index: usize,
    pub value: ElementValue,
}

impl NamedElementValue {
    pub fn name<'a>(&self, constant_pool: &'a ConstantPool) -> Option<&'a str> {
        constant_pool.get_str(self.name_index)
    }
}

impl_read! {
    NamedElementValue(reader) -> Result<NamedElementValue> = {
        let name_index = try!(reader.read_u16::<BigEndian>()) as usize;
        let value = try!(ElementValue::read(reader));

        Ok(NamedElementValue {
            name_index: name_index,
            value: value,
        })
    }
}

impl_print! {
    NamedElementValue(self, printer, constant_pool: &ConstantPool) {
        let name = self.name(constant_pool).expect("Invalid name index");

        try!(printer.write_indent());
        try!(write!(printer, "{}: ", name));
        try!(self.value.print(printer, constant_pool));
    }
}

#[derive(Debug)]
pub enum ElementValue {
    ConstValue(ConstValueInfo),
    EnumConstValue(EnumConstValueInfo),
    Class(ClassInfo),
    Annotation(Annotation),
    Array(Vec<ElementValue>),
}

impl_read! {
    ElementValue(reader) -> Result<ElementValue> = {
        let tag = try!(reader.read_u8());

        let value = match tag {
            b'B' | b'C' | b'D' | b'F' | b'I' | b'J' | b'S' | b'Z' | b's'
                => ElementValue::ConstValue(try!(ConstValueInfo::read(reader))),
            b'e'
                => ElementValue::EnumConstValue(try!(EnumConstValueInfo::read(reader))),
            b'c'
                => ElementValue::Class(try!(ClassInfo::read(reader))),
            b'@'
                => ElementValue::Annotation(try!(Annotation::read(reader))),
            b'['
                => {
                    let values_count = try!(reader.read_u16::<BigEndian>()) as usize;
                    let mut values = Vec::with_capacity(values_count);
                    for _ in 0..values_count {
                        let value = try!(ElementValue::read(reader));
                        values.push(value);
                    }

                    ElementValue::Array(values)
                }
            _ => return Err(Error::BadTagValue(tag)),
        };

        Ok(value)
    }
}

impl_print! {
    ElementValue(self, printer, constant_pool: &ConstantPool) {
        match *self {
            ElementValue::ConstValue(ref info) => try!(info.print(printer, constant_pool)),
            ElementValue::EnumConstValue(ref info) => try!(info.print(printer, constant_pool)),
            ElementValue::Class(ref info) => try!(info.print(printer, constant_pool)),
            ElementValue::Annotation(ref annotation) => try!(annotation.print(&mut printer.by_ref(), constant_pool)),
            ElementValue::Array(ref values) => {
                try!(writeln!(printer, "Array:"));

                for value in values.iter() {
                    try!(printer.write_indent());
                    try!(value.print(&mut printer.sub_indent(1).by_ref(), constant_pool));
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct ConstValueInfo {
    index: usize,
}

impl ConstValueInfo {
    pub fn value<'a>(&self, constant_pool: &'a ConstantPool) -> Option<&'a ConstantPoolEntry> {
        constant_pool.get(self.index)
    }
}

impl_read! {
    ConstValueInfo(reader) -> Result<ConstValueInfo> = {
        let index = try!(reader.read_u16::<BigEndian>()) as usize;

        Ok(ConstValueInfo {
            index: index,
        })
    }
}

impl_print! {
    ConstValueInfo(self, printer, constant_pool: &ConstantPool) {
        let value = self.value(constant_pool).expect("Invalid value index");

        try!(write!(printer, "Constant value: "));
        try!(value.print(printer, constant_pool));
        try!(writeln!(printer, ""));
    }
}

#[derive(Debug)]
pub struct EnumConstValueInfo {
    type_name_index: usize,
    const_name_index: usize,
}

impl EnumConstValueInfo {
    pub fn type_name<'a>(&self, constant_pool: &'a ConstantPool) -> Option<&'a str> {
        constant_pool.get_str(self.type_name_index)
    }

    pub fn const_name<'a>(&self, constant_pool: &'a ConstantPool) -> Option<&'a str> {
        constant_pool.get_str(self.const_name_index)
    }
}

impl_read! {
    EnumConstValueInfo(reader) -> Result<EnumConstValueInfo> = {
        let type_name_index = try!(reader.read_u16::<BigEndian>()) as usize;
        let const_name_index = try!(reader.read_u16::<BigEndian>()) as usize;

        Ok(EnumConstValueInfo {
            type_name_index: type_name_index,
            const_name_index: const_name_index,
        })
    }
}

impl_print! {
    EnumConstValueInfo(self, printer, constant_pool: &ConstantPool) {
        let type_name = self.type_name(constant_pool).expect("Invalid name index");
        let const_name = self.const_name(constant_pool).expect("Invalid name index");

        try!(writeln!(printer, "Enum constant value: {} [{}]", const_name, type_name));
    }
}

#[derive(Debug)]
pub struct ClassInfo {
    index: usize,
}

impl ClassInfo {
    pub fn class<'a>(&self, constant_pool: &'a ConstantPool) -> Option<&'a str> {
        constant_pool.get_str(self.index)
    }
}

impl_read! {
    ClassInfo(reader) -> Result<ClassInfo> = {
        let index = try!(reader.read_u16::<BigEndian>()) as usize;

        Ok(ClassInfo {
            index: index,
        })
    }
}

impl_print! {
    ClassInfo(self, printer, constant_pool: &ConstantPool) {
        let class = self.class(constant_pool).expect("Invalid class index");

        try!(writeln!(printer, "Class: {}", class));
    }
}
