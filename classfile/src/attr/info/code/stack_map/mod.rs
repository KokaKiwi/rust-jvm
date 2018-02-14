use constant::ConstantPool;
use error::Result;
use self::frame::StackMapFrame;

pub mod frame;

#[derive(Debug)]
pub struct StackMapTableAttrInfo {
    entries: Vec<StackMapFrame>,
}

impl_read! {
    StackMapTableAttrInfo(reader, _constant_pool: &ConstantPool) -> Result<Self> = {
        let entries_count = try!(reader.read_u16::<BigEndian>()) as usize;
        let mut entries = Vec::with_capacity(entries_count);
        for _ in 0..entries_count {
            let entry = try!(StackMapFrame::read(reader));
            entries.push(entry);
        }

        Ok(StackMapTableAttrInfo {
            entries: entries,
        })
    }
}

impl_print! {
    StackMapTableAttrInfo(self, printer, _constant_pool: &ConstantPool) {
        for (i, entry) in self.entries.iter().enumerate() {
            try!(entry.print(printer, &self.entries[0..i]));
        }
    }
}
