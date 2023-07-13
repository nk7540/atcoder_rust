use proconio::input;
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`
// no method named `abs_diff` found for type `u32`

fn main() {
    input! {
        n: usize, // 2000
        a: [u32; n], // 10^9
    }
    // loop over pieces as the first piece selected
    // max(a[i] + dp(rest))
    let mut dp = vec![vec![0; n * 2]; n * 2];
    for i in 0..n {
        dp[i][i] = a[i];
    }
    for j in 1..=n {
        for k in 0..n {
            let kj = (k + j) % n;

            dp[k][k + j] = if (n - j) % 2 == 0 {
                // IOI
                if a[k] > a[kj] {
                    dp[k + 1][k + j]
                } else {
                    dp[k][k + j - 1]
                }
            } else {
                // JOI
                (a[k] + dp[k + 1][k + j]).max(a[kj] + dp[k][k + j - 1])
            }
        }
    }
    let ans = (0..n).map(|i| a[i] + dp[i + 1][i + n]).max().unwrap();
    println!("{}", ans);
}
