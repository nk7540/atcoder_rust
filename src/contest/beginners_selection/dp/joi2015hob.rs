use proconio::input;
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`
// no method named `abs_diff` found for type `u32`

fn main() {
    input! {
        n: usize, // 2000
        a: [u64; n], // 10^9
    }
    // loop over pieces as the first piece selected
    // dp[start][end]
    let mut dp = vec![vec![0; n]; n];
    for diff in 0..n {
        for i in 0..n {
            let j = (i + diff) % n;
            let i_inc = (i + 1) % n;
            let j_dec = (j + n - 1) % n;

            dp[i][j] = if (n - diff) % 2 == 0 {
                // IOI
                // 0 if j == 0
                if a[j] > a[i] {
                    dp[i][j_dec]
                } else {
                    dp[i_inc][j]
                }
            } else {
                // JOI
                // a[k] if j == 0
                (dp[i_inc][j] + a[i]).max(dp[i][j_dec] + a[j])
            };
        }
    }
    // max(a[i] + dp(rest))
    // let ans = (0..n)
    //     .map(|i| a[i] + dp[(i + 1) % n][(i + n - 1) % n])
    //     .max()
    //     .unwrap();
    let ans = (0..n).map(|i| dp[i][(i + n - 1) % n]).max().unwrap();
    println!("{}", ans);
}
