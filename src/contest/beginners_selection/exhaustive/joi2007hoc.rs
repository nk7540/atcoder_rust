use std::collections::HashSet;

use itertools::Itertools;
use proconio::input;
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`

fn main() {
    input! {
        n: usize, // 1 <= n <= 3000
        pillars: [(isize, isize); n],
    }
    // to find in O(1)
    let pillars_set: HashSet<_> = pillars.iter().collect();

    // pillars.into_iter().combinations(4) // O(10^3^4)=O(10^12)
    // pillars.into_iter().combinations(2) // O(10^6)
    // find if the edge comprises a square (+-) // O(10^3)
    let mut ans = 0;
    for p in pillars.iter().combinations(2) {
        let (x1, y1) = p[0];
        let (x2, y2) = p[1];
        let dx = x1 - x2;
        let dy = y1 - y2;
        if pillars_set.contains(&(x1 + dy, y1 - dx)) && pillars_set.contains(&(x2 + dy, y2 - dx))
            || pillars_set.contains(&(x1 - dy, y1 + dx))
                && pillars_set.contains(&(x2 - dy, y2 + dx))
        {
            ans = ans.max(dx.pow(2) + dy.pow(2));
        }
    }
    println!("{}", ans);
}
