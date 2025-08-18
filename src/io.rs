use std::io::{self, BufReader, Read, Write};

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

    /// Convert this reader into [std::io::BufReader] with default capacity.
    ///
    /// This does the same thing as `BufReader::new(self)`.
    fn buf_default(self) -> BufReader<Self>;

    /// Convert this reader into [std::io::BufReader].
    ///
    /// This does the same thing as `BufReader::with_capacity(len, self)`.
    fn buf(self, len: usize) -> BufReader<Self>;

    /// Convert this reader into [crate::io::PreRead].
    ///
    /// This does the same thing as `PreRead::new(self)`.
    fn pre<const LEN: usize>(self) -> PreRead<LEN, Self>;

    /// Convert this reader into [crate::str::utf8::Utf8].
    ///
    /// ## Non-blocking IO
    /// Be careful using this with non-blocking IO. While this will
    /// work fine for ASCII strings, there is a chance that [std::io::ErrorKind::WouldBlock]
    /// will be thrown in-between character boundary, resulting in undefined
    /// state of the reader. This may cause a string to appear to have a bunch
    /// of replacement characters where should not have been.
    ///
    /// Consider using [.pre::<4>()](crate::io::PreRead) to make sure it doesn't
    /// happen.
    ///
    /// ## Buffering
    /// While this is implemented for non-buffered readers, this
    /// is highly discouraged as reading multi-byte characters
    /// requires 2 reads (one for first characted, and another
    /// for the rest).
    #[cfg(feature = "str")]
    fn into_utf8(self) -> crate::str::utf8::Utf8<Self>
    where
        Self: Sized;
}
impl<T> ReadExt for T
where
    T: Read,
{
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

    fn buf_default(self) -> BufReader<Self> {
        BufReader::new(self)
    }

    fn buf(self, len: usize) -> BufReader<Self> {
        BufReader::with_capacity(len, self)
    }

    fn pre<const LEN: usize>(self) -> PreRead<LEN, Self> {
        PreRead::new(self)
    }

    #[cfg(feature = "str")]
    fn into_utf8(self) -> crate::str::utf8::Utf8<Self>
    where
        Self: Sized,
    {
        crate::str::utf8::Utf8::new(self)
    }
}

/// Prefetched reader.
///
/// Prereads data in a stack-allocated buffer until reader runs out of data.
///
/// Requesting a read on a 0-lengthed buffer will flush the error.
///
/// ## 0-lengthed prereader
/// If `LEN` is 0, this has no effect and simply calls [std::io::Read::read] on the reader.
///
/// ```
/// use libcommons::prelude::*;
/// use std::io::{self, Read};
///
/// struct R(usize);
/// impl Read for R {
///     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
///         if self.0 > 10 {
///             buf.fill(b'b');
///             Ok(buf.len())
///         } else if self.0 == 10 {
///             self.0 += 1;
///             Ok(0)
///         } else {
///             let len = buf.len().min(10 - self.0);
///             self.0 += len;
///             buf[0..len].fill(b'a');
///             Ok(len)
///         }
///     }
/// }
/// assert!(R(0).read_exact(&mut [0u8; 11]).is_err());
/// {
///     let mut buf = [0u8; 10];
///     let mut r = R(0);
///     assert!(r.read_exact(&mut buf).is_ok());
///     assert!(r.read_exact(&mut buf[0..1]).is_err());
///     assert!(r.read_exact(&mut buf[0..1]).is_ok());
///     assert_eq!(&buf, b"baaaaaaaaa");
/// }
/// {
///     let mut r = R(0).pre::<9>();
///     let mut buf = [0u8; 10];
///     assert!(r.read_exact(&mut buf).is_ok());
///     assert!(r.read_exact(&mut [0; 1]).is_err());
///     assert!(r.read_exact(&mut [0; 1]).is_err());
///     assert_eq!(&buf, b"aaaaaaaaaa");
/// }
///
/// struct A(usize);
/// impl Read for A {
///     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
///         if self.0 > 10 {
///             buf.fill(b'b');
///             Ok(buf.len())
///         } else if self.0 == 10 {
///             self.0 += 1;
///             Err(io::Error::new(io::ErrorKind::WouldBlock, "waiting on io"))
///         } else {
///             let len = buf.len().min(10 - self.0);
///             self.0 += len;
///             buf[0..len].fill(b'a');
///             Ok(len)
///         }
///     }
/// }
///
/// {
///     let mut a = A(0).pre::<4>();
///     let mut buf = [0u8; 4];
///
///     assert!(a.read_exact(&mut buf).is_ok());
///     assert!(a.read_exact(&mut [0; 0]).is_ok());
///
///     assert!(a.read_exact(&mut buf).is_ok());
///     assert!(a.read_exact(&mut [0; 0]).is_err());
///
///     assert!(a.read_exact(&mut buf).is_ok());
///     assert!(a.read_exact(&mut [0; 0]).is_ok());
/// }
/// ```
pub struct PreRead<const LEN: usize, R>
where
    R: Read + ?Sized,
{
    buffer: [u8; LEN],
    len: usize,
    error: Option<io::Error>,
    read: R,
}
impl<const LEN: usize, R> PreRead<LEN, R>
where
    R: Read,
{
    /// Create a new [PreRead].
    pub fn new(read: R) -> Self {
        let mut read = Self {
            buffer: [0; LEN],
            len: 0,
            error: None,
            read,
        };
        if LEN != 0 {
            while read.len < LEN {
                match read.read.read(&mut read.buffer[read.len..]) {
                    Ok(len) => read.len += len,
                    Err(why) => {
                        read.error = Some(why);
                        break;
                    }
                }
            }
        }
        read
    }

    /// Resume this reader.
    ///
    /// By default [PreRead] stops reading once it has encountered
    /// the end of the stream ([Read::read] returning `0`). Calling
    /// this will override that.
    ///
    /// ## Errors
    /// [resume()](Self::resume) will call [flush_error()](Self::flush_error)
    /// before filling the buffer.
    ///
    /// [io::ErrorKind::UnexpectedEof] will be returned if reader will
    /// return `0` again.
    ///
    /// If reader returns an error and no bytes were able to be read,
    /// reader error will be returned.
    pub fn resume(&mut self) -> io::Result<()> {
        self.flush_error()?;
        while self.len < LEN {
            match self.read.read(&mut self.buffer[self.len..]) {
                Ok(0) => {
                    if self.len == 0 {
                        return Err(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            "eof when resuming PreRead",
                        ));
                    } else {
                        break;
                    }
                }
                Ok(read_len) => {
                    self.len += read_len;
                }
                Err(why) => {
                    if self.len == 0 {
                        return Err(why);
                    } else {
                        self.error = Some(why);
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    /// Flush the ligering error and refetch.
    ///
    /// If an error has occured when fetching extra data,
    /// by default it is returned after exhaustion of the
    /// internal buffer. Calling this method will instead
    /// return the error early and fill the buffer to
    /// its full capacity.
    ///
    /// This method may return an error multiple times in
    /// succession if filling the buffer fails, so you may
    /// want to run it on loop.
    pub fn flush_error(&mut self) -> io::Result<()> {
        match self.error.take() {
            None => Ok(()),
            Some(why) => {
                while self.len < LEN {
                    match self.read.read(&mut self.buffer[self.len..]) {
                        Ok(0) => break,
                        Ok(read_len) => {
                            self.len += read_len;
                        }
                        Err(why) => {
                            self.error = Some(why);
                            break;
                        }
                    }
                }
                Err(why)
            }
        }
    }
}
impl<const LEN: usize, R> Read for PreRead<LEN, R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if LEN == 0 {
            return self.read.read(buf);
        }

        match (self.len == 0, buf.len() == 0, &mut self.error) {
            (true, _, error @ Some(_)) | (false, true, error @ Some(_)) => {
                let error = error.take().unwrap();
                while self.len < LEN {
                    match self.read.read(&mut self.buffer[self.len..]) {
                        Ok(0) => break,
                        Ok(read_len) => {
                            self.len += read_len;
                        }
                        Err(why) => {
                            self.error = Some(why);
                            break;
                        }
                    }
                }
                Err(error)
            }
            (true, _, None) | (false, true, None) => Ok(0),
            (false, false, error) => {
                let fetch = self.len == LEN || error.is_some();

                let copy_len = buf.len().min(self.len);
                buf[0..copy_len].copy_from_slice(&self.buffer[0..copy_len]);
                self.len -= copy_len;
                if copy_len != LEN {
                    self.buffer.copy_within(copy_len.., 0);
                }

                if fetch {
                    while self.len != LEN {
                        match self.read.read(&mut self.buffer[self.len..]) {
                            Ok(0) => break,
                            Ok(read_len) => {
                                self.len += read_len;
                            }
                            Err(why) => {
                                self.error = Some(why);
                                break;
                            }
                        }
                    }
                }

                Ok(copy_len)
            }
        }
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        if LEN == 0 {
            return self.read.read_exact(buf);
        }

        if buf.len() == 0 {
            return if let Some(why) = self.error.take() {
                while self.len != self.buffer.len() {
                    match self.read.read(&mut self.buffer[self.len..]) {
                        Ok(0) => break,
                        Ok(read_len) => {
                            self.len += read_len;
                        }
                        Err(why) => {
                            self.error = Some(why);
                            break;
                        }
                    }
                }
                Err(why)
            } else {
                Ok(())
            };
        }

        let mut l = 0;
        while l != buf.len() {
            match self.read(&mut buf[l..])? {
                0 => {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "failed to fill the whole buffer",
                    ));
                }
                x => l += x,
            }
        }

        Ok(())
    }
}
