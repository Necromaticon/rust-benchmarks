// @flag --no-memory-splitting --unroll=10
// @expect error

fn fac(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => n * fac(n - 1),
    }
}

pub fn main() {
    let mut a = 1;
    let n = verifier::nondet!(6u64);
    for i in 1..n + 1 as u64 {
        a *= i;
    }
    verifier::assert_ne!(a, fac(n)); // a should equal 6!
}
