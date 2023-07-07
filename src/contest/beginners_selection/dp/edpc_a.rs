use proconio::input;
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`
// no method named `abs_diff` found for type `u32`

fn main() {
    input! {
        n: usize, // 10^5
        h: [i32; n], // 10^4
    }
    let mut dp = vec![std::i32::MAX; n];
    dp[0] = 0;
    for i in 1..n {
        dp[i] = dp[i].min(dp[i - 1] + (h[i] - h[i - 1]).abs());
        if i > 1 {
            dp[i] = dp[i].min(dp[i - 2] + (h[i] - h[i - 2]).abs());
        }
    }
    println!("{}", dp[n - 1]);
}
