use std::{
    error::Error,
    fmt::{Arguments, Display},
    hash::Hash,
    io::Write,
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[derive(Debug)]
/// Failed to push a character into stack string.
///
/// This error occurs when during a [StackString::push] or
/// [StackString::push_str] new data overflows the buffer.
pub struct PushError;
impl Display for PushError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ran out of space in buffer")
    }
}
impl Error for PushError {}

struct Writer<'a, const CAPACITY: usize>(&'a mut StackString<CAPACITY>);
impl<const CAPACITY: usize> Write for Writer<'_, CAPACITY> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = buf.len().min(CAPACITY - self.0.len);
        self.0.buf[self.0.len..][0..len].copy_from_slice(&buf[0..len]);
        self.0.len += len;
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Stack-allocated string.
pub struct StackString<const CAPACITY: usize> {
    buf: [u8; CAPACITY],
    len: usize,
}
impl<const CAPACITY: usize> StackString<CAPACITY> {
    pub const fn new() -> Self {
        Self {
            buf: [0; CAPACITY],
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub const fn capacity(&self) -> usize {
        CAPACITY
    }

    pub fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result<(), PushError> {
        let len = self.len;
        let mut writer = Writer(self);
        match writer.write_fmt(fmt).map_err(|_| PushError) {
            Ok(x) => Ok(x),
            Err(why) => {
                self.len = len;
                Err(why)
            }
        }
    }

    pub fn push(&mut self, char: char) -> Result<(), PushError> {
        self.write_fmt(format_args!("{char}"))
    }

    pub fn push_str(&mut self, str: &str) -> Result<(), PushError> {
        let len = self.len;
        let mut writer = Writer(self);
        match writer.write_all(str.as_bytes()).map_err(|_| PushError) {
            Ok(x) => Ok(x),
            Err(why) => {
                self.len = len;
                Err(why)
            }
        }
    }

    pub fn as_str(&self) -> &str {
        core::str::from_utf8(self.as_bytes()).unwrap()
    }

    pub fn as_str_mut(&mut self) -> &mut str {
        core::str::from_utf8_mut(self.as_bytes_mut()).unwrap()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buf[0..self.len]
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.buf[0..self.len]
    }
}
impl<const CAPACITY: usize> Default for StackString<CAPACITY> {
    fn default() -> Self {
        Self::new()
    }
}
impl<const CAPACITY: usize> Deref for StackString<CAPACITY> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        core::str::from_utf8(&self.buf[0..self.len]).unwrap()
    }
}
impl<const CAPACITY: usize> DerefMut for StackString<CAPACITY> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        core::str::from_utf8_mut(&mut self.buf[0..self.len]).unwrap()
    }
}
impl<const CAPACITY: usize> AsRef<[u8]> for StackString<CAPACITY> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
impl<const CAPACITY: usize> AsMut<[u8]> for StackString<CAPACITY> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_bytes_mut()
    }
}
impl<const CAPACITY: usize> AsRef<str> for StackString<CAPACITY> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl<const CAPACITY: usize> AsMut<str> for StackString<CAPACITY> {
    fn as_mut(&mut self) -> &mut str {
        self.as_str_mut()
    }
}
impl<const CAPACITY: usize> From<StackString<CAPACITY>> for String {
    fn from(value: StackString<CAPACITY>) -> Self {
        String::from_str(value.as_str()).unwrap()
    }
}
impl<const CAPACITY: usize> FromStr for StackString<CAPACITY> {
    type Err = PushError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut st = Self::new();
        st.push_str(s)?;
        Ok(st)
    }
}
impl<const CAPACITY: usize> TryFrom<String> for StackString<CAPACITY> {
    type Error = PushError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut st = Self::new();
        st.push_str(&value)?;
        Ok(st)
    }
}
impl<const CAPACITY: usize> TryFrom<&'_ str> for StackString<CAPACITY> {
    type Error = PushError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut st = Self::new();
        st.push_str(value)?;
        Ok(st)
    }
}
impl<const CAPACITY: usize> Hash for StackString<CAPACITY> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.as_bytes());
    }
}
impl<const CAPACITY: usize> PartialEq for StackString<CAPACITY> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl<const CAPACITY: usize> Eq for StackString<CAPACITY> {}
impl<const CAPACITY: usize> PartialEq<&'_ str> for StackString<CAPACITY> {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl<const CAPACITY: usize> PartialEq<str> for StackString<CAPACITY> {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl<const CAPACITY: usize> PartialEq<String> for StackString<CAPACITY> {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other
    }
}
