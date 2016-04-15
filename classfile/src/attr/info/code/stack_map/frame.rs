use error::Result;

#[derive(Debug)]
pub struct StackMapFrame {
    tag: u8,
    pub info: StackMapFrameInfo,
}

impl StackMapFrame {
    pub fn offset_delta(&self) -> usize {
        match self.info {
            StackMapFrameInfo::SameFrame                                     => self.tag as usize,
            StackMapFrameInfo::SameLocalsOneStackItemFrame(..)               => self.tag as usize - 64,
            StackMapFrameInfo::SameLocalsOneStackItemFrameExtended(ref info) => info.offset_delta as usize,
            StackMapFrameInfo::ChopFrame(ref info)                           => info.offset_delta as usize,
            StackMapFrameInfo::SameFrameExtended(ref info)                   => info.offset_delta as usize,
            StackMapFrameInfo::AppendFrame(ref info)                         => info.offset_delta as usize,
            StackMapFrameInfo::FullFrame(ref info)                           => info.offset_delta as usize,
        }
    }

    pub fn offset(&self, frames: &[StackMapFrame]) -> usize {
        match frames.split_last() {
            Some((ref last, frames)) => last.offset(frames) + self.offset_delta() + 1,
            None                     => self.offset_delta(),
        }
    }
}

impl_read! {
    StackMapFrame(reader) -> Result<Self> = {
        let tag = try!(reader.read_u8());
        let info = try!(StackMapFrameInfo::read(reader, tag));

        Ok(StackMapFrame {
            tag: tag,
            info: info,
        })
    }
}

impl_print! {
    StackMapFrame(self, printer, frames: &[StackMapFrame]) {
        try!(printer.write_indent());
        try!(writeln!(printer, "{} ({:#x}):", self.info.name(), self.offset(frames)));

        try!(self.info.print(&mut printer.sub_indent(1), self, frames));
    }
}

#[derive(Debug)]
pub enum StackMapFrameInfo {
    SameFrame,
    SameLocalsOneStackItemFrame(SameLocalsOneStackItemFrameInfo),
    SameLocalsOneStackItemFrameExtended(SameLocalsOneStackItemFrameExtendedInfo),
    ChopFrame(ChopFrameInfo),
    SameFrameExtended(SameFrameExtendedInfo),
    AppendFrame(AppendFrameInfo),
    FullFrame(FullFrameInfo),
}

impl StackMapFrameInfo {
    fn name(&self) -> &'static str {
        match *self {
            StackMapFrameInfo::SameFrame                                => "SameFrame",
            StackMapFrameInfo::SameLocalsOneStackItemFrame(..)          => "SameLocalsOneStackItemFrame",
            StackMapFrameInfo::SameLocalsOneStackItemFrameExtended(..)  => "SameLocalsOneStackItemFrameExtended",
            StackMapFrameInfo::ChopFrame(..)                            => "ChopFrame",
            StackMapFrameInfo::SameFrameExtended(..)                    => "SameFrameExtended",
            StackMapFrameInfo::AppendFrame(..)                          => "AppendFrame",
            StackMapFrameInfo::FullFrame(..)                            => "FullFrame",
        }
    }
}

impl_read! {
    StackMapFrameInfo(reader, tag: u8) -> Result<Self> = {
        use error::Error;

        let info = match tag {
            0...63      => StackMapFrameInfo::SameFrame,
            64...127    => StackMapFrameInfo::SameLocalsOneStackItemFrame(
                    try!(SameLocalsOneStackItemFrameInfo::read(reader, tag))
                ),
            128...246   => panic!("Reserved tag"),
            247         => StackMapFrameInfo::SameLocalsOneStackItemFrameExtended(
                    try!(SameLocalsOneStackItemFrameExtendedInfo::read(reader, tag))
                ),
            248...250   => StackMapFrameInfo::ChopFrame(try!(ChopFrameInfo::read(reader, tag))),
            251         => StackMapFrameInfo::SameFrameExtended(
                    try!(SameFrameExtendedInfo::read(reader, tag))
                ),
            252...254   => StackMapFrameInfo::AppendFrame(try!(AppendFrameInfo::read(reader, tag))),
            255         => StackMapFrameInfo::FullFrame(try!(FullFrameInfo::read(reader, tag))),
            _           => return Err(Error::BadTagValue(tag)),
        };

        Ok(info)
    }
}

impl_print! {
    StackMapFrameInfo(self, printer, _frame: &StackMapFrame, _frames: &[StackMapFrame]) {
        try!(printer.write_indent());
        try!(writeln!(printer, "[ data ]"));
    }
}

#[derive(Debug)]
pub struct SameLocalsOneStackItemFrameInfo {
    type_info: (), // TODO: Create VerificationTypeInfo type
}

impl_read! {
    SameLocalsOneStackItemFrameInfo(_reader, _tag: u8) -> Result<Self> = {
        Ok(SameLocalsOneStackItemFrameInfo {
            type_info: (),
        })
    }
}

#[derive(Debug)]
pub struct SameLocalsOneStackItemFrameExtendedInfo {
    offset_delta: u16,
    type_info: (), // TODO: Create VerificationTypeInfo type
}

impl_read! {
    SameLocalsOneStackItemFrameExtendedInfo(reader, _tag: u8) -> Result<Self> = {
        let offset_delta = try!(reader.read_u16::<BigEndian>());

        Ok(SameLocalsOneStackItemFrameExtendedInfo {
            offset_delta: offset_delta,
            type_info: (),
        })
    }
}

#[derive(Debug)]
pub struct ChopFrameInfo {
    offset_delta: u16,
}

impl_read! {
    ChopFrameInfo(reader, _tag: u8) -> Result<Self> = {
        let offset_delta = try!(reader.read_u16::<BigEndian>());

        Ok(ChopFrameInfo {
            offset_delta: offset_delta,
        })
    }
}

#[derive(Debug)]
pub struct SameFrameExtendedInfo {
    offset_delta: u16,
}

impl_read! {
    SameFrameExtendedInfo(reader, _tag: u8) -> Result<Self> = {
        let offset_delta = try!(reader.read_u16::<BigEndian>());

        Ok(SameFrameExtendedInfo {
            offset_delta: offset_delta,
        })
    }
}

#[derive(Debug)]
pub struct AppendFrameInfo {
    offset_delta: u16,
    locals: Vec<()>,
}

impl_read! {
    AppendFrameInfo(reader, _tag: u8) -> Result<Self> = {
        let offset_delta = try!(reader.read_u16::<BigEndian>());

        Ok(AppendFrameInfo {
            offset_delta: offset_delta,
            locals: Vec::new(),
        })
    }
}

#[derive(Debug)]
pub struct FullFrameInfo {
    offset_delta: u16,
    locals: Vec<()>,
    stack: Vec<()>,
}

impl_read! {
    FullFrameInfo(reader, _tag: u8) -> Result<Self> = {
        let offset_delta = try!(reader.read_u16::<BigEndian>());

        Ok(FullFrameInfo {
            offset_delta: offset_delta,
            locals: Vec::new(),
            stack: Vec::new(),
        })
    }
}
