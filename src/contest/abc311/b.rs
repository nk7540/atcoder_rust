use proconio::{input, marker::Chars};
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`
// no method named `abs_diff` found for type `u32`

fn main() {
    input! {
        n: usize,
        d: usize,
        ss: [Chars; n],
    }
    let mut ans = 0;
    let mut tmp = 0;
    for di in 0..d {
        if ss.iter().all(|s| s[di] == 'o') {
            tmp += 1;
            ans = ans.max(tmp);
        } else {
            tmp = 0;
        }
    }
    println!("{}", ans);
}
