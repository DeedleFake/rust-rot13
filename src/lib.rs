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

/// Performs ROT13 on all the bytes in buf.
pub fn rot13_bytes(buf: &mut [u8]) {
    for i in 0..buf.len() {
        buf[i] = match in_range(buf[i]) {
            Some(max) => {
                rotate(buf[i], max)
            },
            None => buf[i],
        };
    }
}

pub fn rot13_string(s: &mut String) {
    unsafe {
        rot13_bytes(s.as_mut_vec());
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    // Darn Rust... Be consistent, please.
    //#[test]
    //fn test_rot13_bytes() {
    //    let tests = vec![
    //        ("This is a test.".as_bytes(), "Guvf vf n grfg.".as_bytes()),
    //    ];

    //    for (ref mut t, ref ex) in tests {
    //        let mut t = t.clone();
    //        rot13_bytes(&mut t);

    //        assert_eq!(t, ex);
    //    }
    //}
}
