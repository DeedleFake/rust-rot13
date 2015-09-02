mod read;
pub use read::*;

fn rotate(mut c: u8, max: u8) -> u8 {
    c += 13;
    if c > max {
        c -= 26;
    }

    c
}
