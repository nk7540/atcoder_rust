use std::ops::Index;

use fixedbitset::FixedBitSet;
use proconio::input;
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`
// no method named `abs_diff` found for type `u32`

fn main() {
    input! {
        n: usize,
        t: usize,
        m: usize,
        hate: [(usize, usize); m],
    }
    let hate = hate.iter().fold(vec![0; n], |mut acc, &h| {
        acc[h.1 - 1] |= 1 << h.0 - 1;
        acc
    });
    let possible_teams = (0..1 << n).fold(FixedBitSet::with_capacity(1 << n), |mut acc, i| {
        // i: n-bit integer representing a team, each bit corresponding to the member of the team
        // acc: (1<<n)-bit bitset, each representing if the team represented by that index is possible (no one hating each other)
        if (0..n).all(|j| i & (1 << j) != 0 && (i & hate[j] == 0)) {
            acc.insert(i);
        }
        acc
    });
    let mut dp = vec![vec![0; t + 1]; 1 << n];
    dp[0][0] = 1; // no member, no team
    for s in 1..1 << n {
        for ss in 0..s {
            // s contains ss
            // s ^ ss (members included in s but not ss) can form a new team
            if (s | ss) != s || !possible_teams[s ^ ss] {
                continue;
            }
            for team in 1..t {
                dp[s][team] += dp[ss][team - 1]
            }
        }
    }
    println!("{}", dp[1 << n - 1][t]);
}
