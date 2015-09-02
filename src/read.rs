use std::io::{Read, Result};

/// A Read that performs ROT13 on data being read through it.
pub struct Rot13Reader<'a, R: 'a + Read> {
    r: &'a mut R,
}

impl<'a, R: 'a + Read> Rot13Reader<'a, R> {
    /// Returns a new Read that wraps the given Read.
    pub fn new(r: &'a mut R) -> Rot13Reader<'a, R> {
        Rot13Reader{
            r: r,
        }
    }
}

impl<'a, R: 'a + Read> Read for Rot13Reader<'a, R> {
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
            let mut r = BufReader::new(t);
            let mut r = Rot13Reader::new(&mut r);

            let mut s = String::new();
            r.read_to_string(&mut s).unwrap();

            assert_eq!(s, ex);
        }
    }
}
