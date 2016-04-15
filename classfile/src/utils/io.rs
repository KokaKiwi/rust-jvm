use std::io::Read;
use std::io::Result;
use utils::vec;

pub trait ReadExt: Read {
    fn read_vec(&mut self, size: usize) -> Result<Vec<u8>> {
        let mut data = unsafe { vec::uninitialized(size) };
        try!(self.read_exact(&mut data));
        Ok(data)
    }
}

impl<T: Read> ReadExt for T {}
