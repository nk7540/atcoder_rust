use proconio::{input, marker::Chars};
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`
// no method named `abs_diff` found for type `u32`

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut ans = 1;
    let mut res = [false; 3];
    for (i, &c) in s.iter().enumerate() {
        let idx = match c {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => unreachable!(),
        };
        res[idx] = true;
        if res.iter().all(|&b| b) {
            ans = i + 1;
            break;
        }
    }
    println!("{}", ans);
}
