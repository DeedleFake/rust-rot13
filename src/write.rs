use std::io::{Write, Result};

pub struct Rot13Writer<W: Write> {
    w: W,
}

impl<W: Write> Rot13Writer<W> {
    pub fn new(w: W) -> Rot13Writer<W> {
        Rot13Writer{
            w: w,
        }
    }
}

impl<W: Write> Write for Rot13Writer<W> {
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

        self.w.write(bufrot.as_slice())
    }

    fn flush(&mut self) -> Result<()> {
        self.w.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Write, BufWriter};

    #[test]
    fn test_write() {
        let tests = vec![
            ("This is a test.".as_bytes(), "Guvf vf n grfg.".to_string()),
        ];

        for (t, ex) in tests {
            let mut s = String::new();
            let mut w = Rot13Writer::new(BufWriter::new(s));
            w.write(t).unwrap();
            assert_eq!(s, ex);
        }
    }
}
