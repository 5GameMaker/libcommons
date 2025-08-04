use std::io::{self, Read, Write};

pub trait ReadExt: Read {
    /// Pipe all contents of self into provided writer.
    fn pipe<const BUF: usize, W>(&mut self, write: W) -> io::Result<()>
    where
        W: Write;

    /// Pipe all contents of self into provided writer.
    fn pipe_with<const BUF: usize, W, F>(&mut self, write: W, cb: F) -> io::Result<()>
    where
        W: Write,
        F: FnMut(u64);

    #[cfg(feature = "str")]
    fn into_utf8(self) -> crate::str::utf8::Utf8<Self>
    where
        Self: Sized;
}
impl<T> ReadExt for T
where
    T: Read,
{
    /// Pipe all contents of self into provided writer.
    fn pipe<const BUF: usize, W>(&mut self, mut write: W) -> io::Result<()>
    where
        W: Write,
    {
        let mut reader = Some(self);
        let mut buf = [0u8; BUF];
        let mut len = 0usize;
        while len > 0 || reader.is_some() {
            if len < buf.len() {
                if let Some(x) = &mut reader {
                    match x.read(&mut buf[len..])? {
                        0 => drop(reader.take()),
                        l => len += l,
                    }
                }
            }
            if len > 0 {
                match write.write(&buf[..len])? {
                    0 => {
                        return Err(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            "unexpected eof",
                        ));
                    }
                    l => len -= l,
                }
            }
        }
        Ok(())
    }

    /// Pipe all contents of self into provided writer.
    fn pipe_with<const BUF: usize, W, F>(&mut self, mut write: W, mut cb: F) -> io::Result<()>
    where
        W: Write,
        F: FnMut(u64),
    {
        let mut reader = Some(self);
        let mut buf = [0u8; BUF];
        let mut len = 0usize;
        let mut download = 0u64;
        while len > 0 || reader.is_some() {
            if len < buf.len() {
                if let Some(x) = &mut reader {
                    match x.read(&mut buf[len..])? {
                        0 => drop(reader.take()),
                        l => len += l,
                    }
                }
            }
            if len > 0 {
                match write.write(&buf[..len])? {
                    0 => {
                        return Err(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            "unexpected eof",
                        ));
                    }
                    l => {
                        len -= l;
                        download += l as u64;
                        cb(download);
                    }
                }
            }
        }
        Ok(())
    }

    #[cfg(feature = "str")]
    fn into_utf8(self) -> crate::str::utf8::Utf8<Self>
    where
        Self: Sized,
    {
        crate::str::utf8::Utf8::new(self)
    }
}
