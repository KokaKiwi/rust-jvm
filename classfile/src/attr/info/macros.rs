
macro_rules! empty_attr_info {
    ($name:ident) => {
        #[derive(Debug)]
        pub struct $name;

        impl $name {
            pub fn read<R: ::std::io::Read>(_reader: &mut R, _constant_pool: &$crate::constant::ConstantPool) -> $crate::error::Result<$name> {
                Ok($name)
            }
        }

        impl_print! {
            $name(self, _printer) {}
        }
    }
}
