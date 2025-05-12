use std::io::{Cursor, Read};

use utf8::Utf8;

pub mod stack;
pub mod utf8;

pub trait AsUtf8<'a> {
    type Inner: Read;

    fn as_utf8(&'a self) -> Utf8<Self::Inner>;
}
impl<'a> AsUtf8<'a> for [u8] {
    type Inner = Cursor<&'a [u8]>;

    fn as_utf8(&'a self) -> Utf8<Self::Inner> {
        Utf8::new(Cursor::new(self))
    }
}

pub trait AsUtf8Mut<'a> {
    type Inner: Read;

    fn as_utf8(&'a mut self) -> Utf8<Self::Inner>;
}
impl<'a, R> AsUtf8Mut<'a> for R
where
    R: Read + 'a,
{
    type Inner = &'a mut R;

    fn as_utf8(&'a mut self) -> Utf8<Self::Inner> {
        Utf8::new(self)
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::{AsUtf8, AsUtf8Mut};

    #[test]
    fn chars_u8() {
        let chars = "Hello🐱!".as_bytes();
        let mut iter = chars.as_utf8();

        assert!(matches!(iter.next(), Some(Ok('H'))));
        assert!(matches!(iter.next(), Some(Ok('e'))));
        assert!(matches!(iter.next(), Some(Ok('l'))));
        assert!(matches!(iter.next(), Some(Ok('l'))));
        assert!(matches!(iter.next(), Some(Ok('o'))));
        assert!(matches!(iter.next(), Some(Ok('🐱'))));
        assert!(matches!(iter.next(), Some(Ok('!'))));

        let mut cursor = Cursor::new(chars);
        let mut iter = cursor.as_utf8();

        assert!(matches!(iter.next(), Some(Ok('H'))));
        assert!(matches!(iter.next(), Some(Ok('e'))));
        assert!(matches!(iter.next(), Some(Ok('l'))));
        assert!(matches!(iter.next(), Some(Ok('l'))));
        assert!(matches!(iter.next(), Some(Ok('o'))));
        assert!(matches!(iter.next(), Some(Ok('🐱'))));
        assert!(matches!(iter.next(), Some(Ok('!'))));
    }
}
