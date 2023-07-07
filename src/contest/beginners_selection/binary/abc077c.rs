use std::cmp::Ordering;

use proconio::input;
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`

fn main() {
    input! {
        n: usize, // O(10^5)
        mut a: [u32; n],
        mut b: [u32; n],
        mut c: [u32; n],
    }
    // naive: O(n^3)=O(10^15)
    // sort: O(nlogn * 3)
    // for each a[i], find partition in B, then in C: O(n(logn*2))
    a.sort_unstable();
    b.sort_unstable();
    c.sort_unstable();
    let mut ans = 0;
    for ai in a {
        // find partition in B
        b.binary_search(&ai)
    }
}
