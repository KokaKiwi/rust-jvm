#[macro_use] mod macros;
pub mod classfile;
pub mod code;
pub mod field;
pub mod method;
pub mod misc;

use error::Result;
use constant::ConstantPool;

#[derive(Debug)]
pub enum AttrInfo {
    // Class file
    SourceFile(classfile::SourceFileAttrInfo),
    InnerClasses(classfile::InnerClassesAttrInfo),
    EnclosingMethod(classfile::EnclosingMethodAttrInfo),
    SourceDebugExtension(classfile::SourceDebugExtensionAttrInfo),
    // BootstrapMethods

    // Field
    ConstantValue(field::ConstantValueAttrInfo),

    // Method
    Code(method::CodeAttrInfo),
    Exceptions(method::ExceptionsAttrInfo),
    // RuntimeVisibleParameterAnnotations
    // RuntimeInvisibleParameterAnnotations
    // AnnotationDefault
    // MethodParameters

    // Code
    LineNumberTable(code::LineNumberTableAttrInfo),
    LocalVariableTable(code::LocalVariableTableAttrInfo),
    // LocalVariableTypeTable
    StackMapTable(code::StackMapTableAttrInfo),

    // Misc
    Syncthetic(misc::SyntheticAttrInfo),
    Deprecated(misc::DeprecatedAttrInfo),
    Signature(misc::SignatureAttrInfo),
    RuntimeVisibleAnnotations(misc::RuntimeVisibleAnnotationsAttrInfo),
    // RuntimeInvisibleAnnotations
    // RuntimeVisibleTypeAnnotations
    // RuntimeInvisibleTypeAnnotations

    // Unknown
    Unknown(Vec<u8>),
}

macro_rules! read_by_name {
    ($name_expr:expr, $reader:expr, $pool:expr =>
        $($name:ident => $read:path),*
    ) => {
        match $name_expr {
            $(stringify!($name) => return $read(&mut $reader, $pool).map(AttrInfo::$name),)*
            _ => {}
        }
    };
    ($name_expr:expr, $reader:expr, $pool:expr =>
        $($name:ident => $read:path),+,
    ) => (
        read_by_name!($name_expr, $reader, $pool =>
            $($name => $read),+
        )
    );
}

impl_read! {
    AttrInfo(reader, name: &str, pool: &ConstantPool) -> Result<AttrInfo> = {
        use utils::io::ReadExt;

        let size = try!(reader.read_u32::<BigEndian>()) as usize;
        let data = try!(reader.read_vec(size));

        {
            use std::io::Cursor;

            let mut reader: Cursor<&[u8]> = Cursor::new(&data);

            read_by_name!(name, reader, pool =>
                // Class file
                InnerClasses => classfile::InnerClassesAttrInfo::read,
                SourceFile => classfile::SourceFileAttrInfo::read,
                EnclosingMethod => classfile::EnclosingMethodAttrInfo::read,
                SourceDebugExtension => classfile::SourceDebugExtensionAttrInfo::read,

                // Field
                ConstantValue => field::ConstantValueAttrInfo::read,

                // Method
                Code => method::CodeAttrInfo::read,
                Exceptions => method::ExceptionsAttrInfo::read,

                // Code
                StackMapTable => code::StackMapTableAttrInfo::read,
                LineNumberTable => code::LineNumberTableAttrInfo::read,
                LocalVariableTable => code::LocalVariableTableAttrInfo::read,

                // Misc
                Syncthetic => misc::SyntheticAttrInfo::read,
                Deprecated => misc::DeprecatedAttrInfo::read,
                RuntimeVisibleAnnotations => misc::RuntimeVisibleAnnotationsAttrInfo::read,
                Signature => misc::SignatureAttrInfo::read,
            );
        }

        Ok(AttrInfo::Unknown(data))
    }
}

impl_print! {
    AttrInfo(self, printer, constant_pool: &ConstantPool) {
        match *self {
            // Class files
            AttrInfo::SourceFile(ref info) => try!(info.print(printer, constant_pool)),
            AttrInfo::InnerClasses(ref info) => try!(info.print(printer, constant_pool)),
            AttrInfo::EnclosingMethod(ref info) => try!(info.print(printer, constant_pool)),
            AttrInfo::SourceDebugExtension(ref info) => try!(info.print(printer, constant_pool)),

            // Field
            AttrInfo::ConstantValue(ref info) => try!(info.print(printer, constant_pool)),

            // Method
            AttrInfo::Code(ref info) => try!(info.print(printer, constant_pool)),
            AttrInfo::Exceptions(ref info) => try!(info.print(printer, constant_pool)),

            // Code
            AttrInfo::StackMapTable(ref info) => try!(info.print(printer, constant_pool)),
            AttrInfo::LineNumberTable(ref info) => try!(info.print(printer)),
            AttrInfo::LocalVariableTable(ref info) => try!(info.print(printer, constant_pool)),

            // Misc
            AttrInfo::Syncthetic(ref info) => try!(info.print(printer)),
            AttrInfo::Deprecated(ref info) => try!(info.print(printer)),
            AttrInfo::RuntimeVisibleAnnotations(ref info) => try!(info.print(printer, constant_pool)),
            AttrInfo::Signature(ref info) => try!(info.print(printer, constant_pool)),

            // Unknown
            AttrInfo::Unknown(ref data) => {
                try!(printer.write_indent());
                try!(writeln!(printer, "Unknown [ {} bytes ]", data.len()))
            }
        }
    }
}
