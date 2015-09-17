use std::default::Default;
use std::fmt;
use std::io;
use std::io::{Result, Write};

pub struct Printer<W: Write, C = ()> {
    writer: W,
    pub context: C,
    pub indent_size: usize,
    pub indent_char: char,
}

impl<W: Write, C: Copy = ()> Printer<W, C> {
    pub fn new(writer: W, context: C) -> Printer<W, C> {
        Printer {
            writer: writer,
            context: context,
            indent_size: 0,
            indent_char: ' ',
        }
    }

    pub fn sub(&mut self) -> Printer<&mut W, C> {
        let context = self.context;
        self.sub_context(context)
    }

    pub fn sub_context<C1>(&mut self, context: C1) -> Printer<&mut W, C1> {
        Printer {
            writer: self.writer.by_ref(),
            context: context,
            indent_size: self.indent_size,
            indent_char: self.indent_char,
        }
    }

    pub fn with_indent(&mut self, amount: isize) -> &mut Self {
        self.indent_size = (self.indent_size as isize + amount) as usize;
        self
    }

    pub fn indent(&mut self) -> Result<()> {
        let c = self.indent_char;
        for _ in 0..self.indent_size {
            try!(write!(self, "{}", c));
        }
        Ok(())
    }

    pub fn print<P: Print<C>>(&mut self, value: P) -> Result<()> {
        value.dump(self)
    }
}

impl<W: Write, C> Write for Printer<W, C> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.writer.write_all(buf)
    }

    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()> {
        self.writer.write_fmt(fmt)
    }
}

impl Default for Printer<io::Stdout> {
    fn default() -> Printer<io::Stdout> {
        Printer::new(io::stdout(), ())
    }
}

pub trait Print<C = ()> {
    fn dump<W: Write>(&self, printer: &mut Printer<W, C>) -> Result<()>;
}

impl<'a, C, T: Print<C>> Print<C> for &'a T {
    fn dump<W: Write>(&self, printer: &mut Printer<W, C>) -> Result<()> {
        (*self).dump(printer)
    }
}
