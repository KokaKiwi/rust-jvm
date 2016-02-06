use constant::ConstantPool;
use error::Result;

#[derive(Debug)]
pub struct SignatureAttrInfo {
    index: usize,
}

impl SignatureAttrInfo {
    pub fn value<'a>(&self, constant_pool: &'a ConstantPool) -> Option<&'a str> {
        constant_pool.get_str(self.index)
    }
}

impl_read! {
    SignatureAttrInfo(reader, _constant_pool: &ConstantPool) -> Result<SignatureAttrInfo> = {
        let index = try!(reader.read_u16::<BigEndian>()) as usize;

        Ok(SignatureAttrInfo {
            index: index,
        })
    }
}

impl_print! {
    SignatureAttrInfo(self, printer, constant_pool: &ConstantPool) {
        let value = self.value(constant_pool).expect("Invalid signature index");

        try!(printer.write_indent());
        try!(writeln!(printer, "{:?}", value));
    }
}
