use std::cmp::Ordering;

use proconio::input;
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`

fn main() {
    input! {
        d: usize, // total length of ring road
        n: usize, // number of stores: O(10^6)
        m: usize, // number of orders: O(10^5)
        mut stores: [usize; n - 1],
        recipients: [usize; m],
    }
    // find the closest store for each recipient (compute the distance at the same time)
    // and sum them up
    // naive solution: O(10^6) for each O(10^5)
    // sort stores once, then binary search for each recipient: O((n+m)log(n))
    stores.push(0); // S1
    stores.push(d); // S1
    stores.sort_unstable(); // [usize; n + 1]

    let mut ans = 0;
    // for each recipient
    'recipient: for r in recipients {
        let mut low = 0;
        let mut high = n + 1; // stores.len() (exclusive upper bound)
        let mut mid = (low + high) / 2; // round down
                                        // when low becomes equal to high, there is no match (because high is exclusive)
        while low < high {
            match stores[mid].cmp(&r) {
                Ordering::Equal => continue 'recipient,
                Ordering::Greater => high = mid,
                Ordering::Less => low = mid + 1,
            }
            // placed here to update mid in the last round before low becomes equal to high
            mid = (low + high) / 2;
        }
        ans += (stores[mid] - r).min(r - stores[mid - 1]);
    }
    println!("{}", ans);
}
