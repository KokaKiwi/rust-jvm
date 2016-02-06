// Util macros

macro_rules! impl_read {
    ($name:ident($reader:ident) -> $result:ty = $body:block) => {
        impl $name {
            #[allow(unused_imports)]
            pub fn read<R: ::std::io::Read>($reader: &mut R) -> $result {
                use byteorder::{ReadBytesExt, BigEndian};
                $body
            }
        }
    };
    ($name:ident($reader:ident, $($argname:ident: $argty:ty),+) -> $result:ty = $body:block) => {
        impl $name {
            #[allow(unused_imports)]
            pub fn read<R: ::std::io::Read>($reader: &mut R,
                                            $($argname: $argty),+) -> $result
            {
                use byteorder::{ReadBytesExt, BigEndian};
                $body
            }
        }
    };
}
