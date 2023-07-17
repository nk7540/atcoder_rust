use std::collections::HashSet;

use proconio::{input, marker::Chars};
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`
// no method named `abs_diff` found for type `u32`

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let mut set = HashSet::new();
    for si in s {
        let mut reversed = si.clone();
        reversed.reverse();
        set.insert(if si < reversed { si } else { reversed });
    }
    println!("{}", set.len());
}
