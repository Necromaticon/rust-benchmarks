// @flag --integer-overflow
// @expect error

pub fn main() {
    let a: u8 = 128;
    let b: u8 = 129;
    let c = a - b;
}
