use constant::ConstantPool;
use error::Result;
// use byteorder::{ReadBytesExt, BigEndian};
use std::io::Read;

#[derive(Debug)]
pub struct StackMapTableAttrInfo;

// TODO
impl StackMapTableAttrInfo {
    pub fn read<R: Read>(_reader: &mut R, _pool: &ConstantPool) -> Result<Self> {
        Ok(StackMapTableAttrInfo)
    }
}

impl_print! {
    StackMapTableAttrInfo(self, _printer, _constant_pool: &ConstantPool) {
    }
}
