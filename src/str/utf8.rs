use std::io::Read;

/// An iterator over UTF-8 characters from a [Read].
///
/// ```
/// use libcommons::str::utf8::Utf8;
/// use std::io::Cursor;
///
/// let s = b"Hello, world! \xF0\x9F\xA6\x80";
///
/// assert_eq!(Utf8::new(Cursor::new(s)).count(), 15);
/// assert_eq!(Utf8::new(Cursor::new(s)).last().unwrap().unwrap(), '🦀');
/// ```
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
/// ```
/// use libcommons::prelude::*;
/// use std::io::{Read, Error, ErrorKind, Result};
///
/// let original = "Hello, world! 🦀";
/// let s = b"Hello, world! \xF0\x9FX\xA6\x80"; // Using X to indicate where the interrupt should go.
///
/// struct A<'a> {
///     buf: &'a [u8],
/// };
/// impl<'a> A<'a> {
///     fn new(buf: &'a [u8]) -> Self {
///         Self { buf }
///     }
/// }
/// impl<'a> Read for A<'a> {
///     fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
///         let request_len = self.buf.len().min(buf.len());
///         let mut copy_buf = &self.buf[0..request_len];
///
///         if let Some(i) = copy_buf.iter().enumerate().find(|(_, x)| **x == b'X').map(|(i, _)| i) {
///             if i == 0 {
///                 self.buf = &self.buf[1..];
///                 return Err(Error::new(ErrorKind::WouldBlock, "doing some io in background"));
///             }
///
///             copy_buf = &copy_buf[..i];
///         }
///
///         buf[0..copy_buf.len()].copy_from_slice(copy_buf);
///
///         self.buf = &self.buf[copy_buf.len()..];
///
///         Ok(copy_buf.len())
///     }
/// }
///
/// let mut compare = String::new();
///
/// let mut naive = A::new(s).into_utf8();
/// for char in naive {
///     match char {
///         Ok(x) => compare.push(x),
///         Err(_) => (),
///     }
/// }
///
/// assert_ne!(compare, original);
///
/// compare.clear();
///
/// let mut checked = A::new(s).pre::<4>().into_utf8();
/// // Reading a 0-length slice on a `PreRead` flushes the error.
/// while checked.inner_mut().flush_error().is_err() {}
/// while let Some(char) = checked.next() {
///     compare.push(char.unwrap());
///     while checked.inner_mut().flush_error().is_err() {}
/// }
///
/// assert_eq!(compare, original);
/// ```
///
/// ## Buffering
/// While this is implemented for non-buffered readers, this
/// is highly discouraged as reading multi-byte characters
/// requires 2 reads (one for first characted, and another
/// for the rest).
pub struct Utf8<R: Read>(R);
impl<R: Read> Utf8<R> {
    pub fn new(read: R) -> Self {
        Self(read)
    }

    pub fn into_inner(self) -> R {
        self.0
    }

    pub fn inner(&self) -> &R {
        &self.0
    }
    pub fn inner_mut(&mut self) -> &mut R {
        &mut self.0
    }
}
impl<R: Read> Iterator for Utf8<R> {
    type Item = std::io::Result<char>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0];

        match self.0.read(&mut buf) {
            Ok(0) => return None,
            Ok(_) => (),
            Err(why) => return Some(Err(why)),
        }
        let char1 = buf[0];

        if char1.is_ascii() {
            return Some(Ok(char1 as char));
        }

        let ones = char1.leading_ones();

        let len = if !(2..=4).contains(&ones) {
            return Some(Ok(char::REPLACEMENT_CHARACTER));
        } else {
            ones as usize - 1
        };

        let mut buf = [0; 3];
        let buf = match self.0.read(&mut buf[..len]) {
            Ok(0) => return None,
            Ok(x) if x != len => return Some(Ok(char::REPLACEMENT_CHARACTER)),
            Ok(x) => &buf[..x],
            Err(why) => return Some(Err(why)),
        };

        let mut final_char = char1 as u32 & (0b00111111 >> len);
        for v in buf {
            if *v & 0b11000000 != 0b10000000 {
                return Some(Ok(char::REPLACEMENT_CHARACTER));
            }
            final_char <<= 6;
            final_char |= *v as u32 & 0b00111111;
        }

        Some(Ok(
            char::from_u32(final_char).unwrap_or(char::REPLACEMENT_CHARACTER)
        ))
    }
}
