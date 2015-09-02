use std::io::{Read, BufReader, Result};

struct Rot13<R: Read> {
    r: R,
}

impl<R: Read> Rot13<R> {
    fn new(r: R) -> Rot13<R> {
        Rot13{
            r: r,
        }
    }
}

impl<R: Read> Read for Rot13<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
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

fn rotate(mut c: u8, max: u8) -> u8 {
    c += 13;
    if c > max {
        c -= 26;
    }
    
    c
}

fn main() {
    let mut r = Rot13::new(BufReader::new("Pna V unir fbzr pnxr, cyrnfr?".as_bytes()));
    
    let mut s = String::new();
    r.read_to_string(&mut s).unwrap();
    println!("{}", s);
}
