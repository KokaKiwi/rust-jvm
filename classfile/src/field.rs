use byteorder::{ReadBytesExt, BigEndian};
use attr::Attr;
use constant::ConstantPool;
use error::{Result, Error};
use std::io::Read;

#[derive(Debug)]
pub struct FieldInfo {
    pub access_flags: flags::AccessFlags,
    name_index: usize,
    desc_index: usize,
    pub attrs: Vec<Attr>,
}

impl FieldInfo {
    pub fn read<R: Read>(reader: &mut R, cp: &ConstantPool) -> Result<FieldInfo> {
        // Read access flags
        let access_flags = try!(reader.read_u16::<BigEndian>());
        let access_flags = match flags::AccessFlags::from_bits(access_flags) {
            Some(flags) => flags,
            None => return Err(Error::BadAccessFlags(access_flags)),
        };

        // Read indexes
        let name_index = try!(reader.read_u16::<BigEndian>()) as usize;
        let desc_index = try!(reader.read_u16::<BigEndian>()) as usize;

        // Read attributes
        let attrs_count = try!(reader.read_u16::<BigEndian>()) as usize;
        let mut attrs = Vec::with_capacity(attrs_count);
        for _ in 0..attrs_count {
            let attr = try!(Attr::read(reader, cp));
            attrs.push(attr);
        }

        Ok(FieldInfo {
            access_flags: access_flags,
            name_index: name_index,
            desc_index: desc_index,
            attrs: attrs,
        })
    }

    pub fn name<'a>(&self, pool: &'a ConstantPool) -> Option<&'a str> {
        pool.get_str(self.name_index)
    }

    pub fn desc<'a>(&self, pool: &'a ConstantPool) -> Option<&'a str> {
        pool.get_str(self.desc_index)
    }
}

impl_print! {
    FieldInfo(self, printer, constant_pool: &ConstantPool) {
        let name = self.name(constant_pool).expect("Invalid name index");
        let desc = self.desc(constant_pool).expect("Invalid desc index");

        try!(printer.write_indent());
        try!(writeln!(printer, "Field `{}` [{}]:", name, desc));

        {
            let mut printer = printer.sub_indent(1);

            try!(printer.write_indent());
            try!(writeln!(printer, "Access flags: {:?}", self.access_flags));

            try!(printer.write_indent());
            try!(writeln!(printer, "Attrs:"));
            for attr in self.attrs.iter() {
                try!(attr.print(&mut printer.sub_indent(1), constant_pool));
            }
        }
    }
}

pub mod flags {
    bitflags! {
        pub flags AccessFlags: u16 {
            #[doc = "Declared public; may be accessed from outside its package."]
            const ACC_PUBLIC      = 0x0001,
            #[doc = "Declared private; usable only within the defining class."]
            const ACC_PRIVATE     = 0x0002,
            #[doc = "Declared protected; may be accessed within subclasses."]
            const ACC_PROTECTED   = 0x0004,
            #[doc = "Declared static."]
            const ACC_STATIC      = 0x0008,
            #[doc = "Declared final; never directly assigned to after object construction (JLS §17.5)."]
            const ACC_FINAL       = 0x0010,
            #[doc = "Declared volatile; cannot be cached."]
            const ACC_VOLATILE    = 0x0040,
            #[doc = "Declared transient; not written or read by a persistent object manager."]
            const ACC_TRANSIENT   = 0x0080,
            #[doc = "Declared synthetic; not present in the source code."]
            const ACC_SYNTHETIC   = 0x1000,
            #[doc = "Declared as an element of an enum."]
            const ACC_ENUM        = 0x4000,
        }
    }
}
