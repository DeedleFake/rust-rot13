use std::io::{Read, Result};

/// A Read that performs ROT13 on data being read through it.
pub struct Rot13Reader<R: Read> {
    r: R,
}

impl<R: Read> Rot13Reader<R> {
    /// Returns a new Read that wraps the given Read.
    pub fn new(r: R) -> Rot13Reader<R> {
        Rot13Reader{
            r: r,
        }
    }
}

impl<R: Read> Read for Rot13Reader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let r = match self.r.read(buf) {
            Ok(r) => r,
            Err(err) => {
                return Err(err);
            }
        };

        super::rot13(buf);

        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, BufReader};

    #[test]
    fn test_read() {
        let tests = vec![
            ("This is a test.".as_bytes(), "Guvf vf n grfg.".to_string()),
        ];

        for (t, ex) in tests {
            let mut r = Rot13Reader::new(BufReader::new(t));
            let mut s = String::new();
            r.read_to_string(&mut s).unwrap();
            assert_eq!(s, ex);
        }
    }
}
