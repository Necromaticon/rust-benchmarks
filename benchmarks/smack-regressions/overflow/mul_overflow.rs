// @flag --integer-overflow
// @expect overflow

pub fn main() {
    let a: u8 = 128;
    let b: u8 = 2;
    let c = a * b;
}
