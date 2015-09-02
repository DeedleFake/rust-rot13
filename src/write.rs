use std::io::{Write, Result};

pub struct Rot13Writer<'a, W: 'a + Write> {
    w: &'a mut W,
}

impl<'a, W: 'a + Write> Rot13Writer<'a, W> {
    pub fn new(w: &'a mut W) -> Rot13Writer<'a, W> {
        Rot13Writer{
            w: w,
        }
    }
}

impl<'a, W: 'a + Write> Write for Rot13Writer<'a, W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        use super::{in_range, rotate};

        let mut bufrot: Vec<u8> = Vec::with_capacity(buf.len());
        for c in buf {
            let c = *c;
            let c = match in_range(c) {
                Some(max) => {
                    rotate(c, max)
                },

                None => c,
            };

            bufrot.push(c);
        }

        self.w.write(&bufrot)
    }

    fn flush(&mut self) -> Result<()> {
        self.w.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Write};

    #[test]
    fn test_write() {
        let tests = vec![
            ("This is a test.".as_bytes(), "Guvf vf n grfg.".to_string()),
        ];

        for (t, ex) in tests {
            let mut s: Vec<u8> = Vec::new();
            {
                let mut w = Rot13Writer::new(&mut s);
                w.write(t).unwrap();
            }
            assert_eq!(String::from_utf8(s).unwrap(), ex);
        }
    }
}
