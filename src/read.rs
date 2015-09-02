use std::io::{Read, Result};

/// A Read that performs ROT13 on data being read through it.
pub struct Rot13<R: Read> {
    r: R,
}

impl<R: Read> Rot13<R> {
    /// Returns a new Read that wraps the given Read.
    pub fn new(r: R) -> Rot13<R> {
        Rot13{
            r: r,
        }
    }
}

impl<R: Read> Read for Rot13<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        use super::rotate;

        let r = match self.r.read(buf) {
            Ok(r) => r,
            Err(err) => {
                return Err(err);
            }
        };

        for i in 0..buf.len() {
            let c = buf[i] as char;
            if (c >= 'A') && (c <= 'Z') {
                buf[i] = rotate(buf[i], 'Z' as u8);
            }
            else if (c >= 'a') && (c <= 'z') {
                buf[i] = rotate(buf[i], 'z' as u8);
            }
        }

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
            let mut r = Rot13::new(BufReader::new(t));
            let mut s = String::new();
            r.read_to_string(&mut s).unwrap();
            assert_eq!(s, ex);
        }
    }
}
