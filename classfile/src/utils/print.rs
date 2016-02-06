#![allow(dead_code)]
use std::default::Default;
use std::fmt;
use std::io;

pub struct Printer<W: io::Write> {
    writer: W,
    pub indent_count: usize,
    pub indent_str: &'static str,
}

impl<W: io::Write> Printer<W> {
    pub fn new(writer: W) -> Printer<W> {
        Printer {
            writer: writer,
            indent_count: 0,
            indent_str: "  ",
        }
    }

    pub fn by_ref(&mut self) -> Printer<&mut io::Write> {
        Printer {
            writer: &mut self.writer as &mut io::Write,
            indent_count: self.indent_count,
            indent_str: self.indent_str,
        }
    }

    pub fn sub(&mut self) -> Printer<&mut W> {
        Printer {
            writer: self.writer.by_ref(),
            indent_count: self.indent_count,
            indent_str: self.indent_str,
        }
    }

    pub fn sub_indent(&mut self, amount: isize) -> Printer<&mut W> {
        let mut sub = self.sub();
        sub.with_indent(amount);
        sub
    }

    pub fn with_indent(&mut self, amount: isize) -> &mut Self {
        self.indent_count = (self.indent_count as isize + amount) as usize;
        self
    }

    pub fn write_indent(&mut self) -> io::Result<()> {
        for _ in 0..self.indent_count {
            try!(write!(self.writer, "{}", self.indent_str));
        }
        Ok(())
    }
}

impl<W: io::Write> io::Write for Printer<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.writer.write_all(buf)
    }

    fn write_fmt(&mut self, fmt: fmt::Arguments) -> io::Result<()> {
        self.writer.write_fmt(fmt)
    }
}

impl Default for Printer<io::Stdout> {
    fn default() -> Printer<io::Stdout> {
        Printer::new(io::stdout())
    }
}

macro_rules! impl_print {
    ($name:ident($selfname:ident, $printer:ident) $block:block) => {
        impl $name {
            #[allow(unused_imports)]
            pub fn print<W: ::std::io::Write>(&$selfname,
                                              $printer: &mut $crate::utils::print::Printer<W>)
                -> ::std::io::Result<()>
            {
                use ::std::io::prelude::*;
                $block
                Ok(())
            }
        }
    };
    ($name:ident($selfname:ident, $printer:ident, $($argname:ident: $argty:ty),+) $block:block) => {
        impl $name {
            #[allow(unused_imports)]
            pub fn print<W: ::std::io::Write>(&$selfname,
                                              $printer: &mut $crate::utils::print::Printer<W>,
                                              $($argname: $argty),+)
                -> ::std::io::Result<()>
            {
                use ::std::io::prelude::*;
                $block
                Ok(())
            }
        }
    };
}
