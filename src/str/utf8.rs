use std::io::Read;

pub struct Utf8<R: Read>(R);
impl<R: Read> Utf8<R> {
    pub fn new(read: R) -> Self {
        Self(read)
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
