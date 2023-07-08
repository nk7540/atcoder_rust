use proconio::input;
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`
// no method named `abs_diff` found for type `u32`

fn main() {
    input! {
        n: usize,
        k: u32,
        mut ab: [(usize, u32); n],
    }
    ab.sort_unstable_by(|(a1, _), (a2, _)| a2.cmp(a1));
    let mut ans = 1;
    let mut sum = 0;
    for (a, b) in ab.iter() {
        sum += b;
        if sum > k {
            ans = a + 1;
            break;
        }
    }
    println!("{}", ans);
}
