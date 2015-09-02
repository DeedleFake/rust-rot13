mod read;
pub use read::*;

mod write;
pub use write::*;

fn rotate(mut c: u8, max: u8) -> u8 {
    c += 13;
    if c > max {
        c -= 26;
    }

    c
}

fn in_range(c: u8) -> Option<u8> {
    let r = if (c >= 'A' as u8) && (c <= 'Z' as u8) {
        Some('Z' as u8)
    } else if (c >= 'a' as u8) && (c <= 'z' as u8) {
        Some('z' as u8)
    } else {
        None
    };

    r
}

pub fn rot13(buf: &mut [u8]) {
    for i in 0..buf.len() {
        buf[i] = match in_range(buf[i]) {
            Some(max) => {
                rotate(buf[i], max)
            },
            None => buf[i],
        };
    }
}
