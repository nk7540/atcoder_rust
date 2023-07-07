use proconio::input;
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`

fn main() {
    input! {
        n: usize, // 10^5
        k: usize,
        h: [i32; n], // 10^4
    }
    let mut dp = vec![std::i32::MAX; n];
    dp[0] = 0;
    for i in 0..n - 1 {
        for ik in 1..(k + 1).min(n - i) {
            dp[i + ik] = dp[i + ik].min(dp[i] + (h[i + ik] - h[i]).abs());
        }
    }
    println!("{}", dp[n - 1]);
}
