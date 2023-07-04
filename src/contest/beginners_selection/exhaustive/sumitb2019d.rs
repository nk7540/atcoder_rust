use itertools::Itertools;
use proconio::{input, marker::Chars};
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    // cannot loop over all
    // only need to loop (n, 3) times
    // use .combinations() -> still got TLE (O(n^3)=O(10^15))
    // reverse thinking: explore all combinations of secret numbers (000~999): O(10^3)
    // finding whether each number is included is O(N)=O(10^5)
    let mut ans = 0;
    for num in 0..1000 {
        let num_str = format!("{:03}", num);
        let mut idx = 0;
        for &c in s.iter() {
            if c == num_str.chars().nth(idx).unwrap() {
                idx += 1;
            }
            if idx == 3 {
                ans += 1;
                break;
            }
        }
    }
    println!("{}", ans);
    // let numbers: Vec<Vec<char>> = s.into_iter().combinations(3).collect();
    // println!("{}", numbers.into_iter().unique().count());
}
