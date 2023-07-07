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
    // for each a[i], find partition in B
    // then for each b[j] with j greater than the partition, find partition in C
    // total: O(nlogn + n*nlogn) // still got TLE
    a.sort_unstable();
    b.sort_unstable();
    c.sort_unstable();

    let mut ans = 0;
    for bi in b {
        // obtain the index of the first element of the second partition (gteq bi)
        //
        // binary_search returns Err(left) if Equal has not been encountered
        // `left` will be the index where the target x should be inserted
        // in other words, v[left - 1] < x && x < v[left]
        //
        // now we want the index whose value is greater than x even if there is a match
        // if the case v[mid] == x is treated as Less, it achieves the greatness
        let a_gteq_bi = a
            .binary_search_by(|mid_v| {
                if mid_v < &bi {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .unwrap_or_else(|i| i);
        // obtain the index of the first element of the second partition (gt bi)
        let c_gt_bi = c
            .binary_search_by(|mid_v| {
                if mid_v <= &bi {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .unwrap_or_else(|i| i);
        ans += a_gteq_bi * (n - c_gt_bi);
    }
    println!("{}", ans);
}
