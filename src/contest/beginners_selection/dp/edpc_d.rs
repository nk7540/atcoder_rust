use proconio::input;
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`
// no method named `abs_diff` found for type `u32`

fn main() {
    input! {
        n: usize, // 10^2
        w: usize, // 10^5
        wv: [(u64, u64); n] // w, 10^9
    }
    // extra row for the case of 0 items
    // dp[0] is initialized as well
    let mut dp = vec![vec![0; w + 1]; n + 1];
    // indices of dp to be updated
    for i in 1..=n {
        let (wi, vi) = wv[i - 1]; // CURRENT item (wv[i - 1] is covered by dp[i])
        for iw in 0..=w {
            if wi <= iw as u64 {
                dp[i][iw] = dp[i][iw].max(dp[i - 1][iw - wi as usize] + vi);
            }
            dp[i][iw] = dp[i][iw].max(dp[i - 1][iw]);
        }
    }
    println!("{}", dp[n][w]);
}
