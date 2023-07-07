use proconio::input;
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`

const INF: i32 = std::i32::MIN / 3;

fn main() {
    input! {
        n: usize, // 10^5
        abc: [[i32; 3]; n], // 10^4
    }
    let mut dp = vec![vec![INF; 3]; n];
    dp[0] = abc[0].clone();
    // indices of dp to be updated
    for i in 1..n {
        // indices of each of dp to be updated
        for j in 0..3 {
            // indices of each of dp to be referenced
            for k in 0..3 {
                if j == k {
                    continue;
                }
                dp[i][j] = dp[i][j].max(dp[i - 1][k] + abc[i][j]);
            }
        }
    }
    println!("{}", dp[n - 1].iter().max().unwrap());
}
