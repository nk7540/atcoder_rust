use itertools::Itertools;
use proconio::input;
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`
// no method named `abs_diff` found for type `u32`

fn main() {
    input! {
        n: usize,
        m: usize, // 1 <= c <= m
    }
    let items: Vec<_> = (0..n)
        .map(|_| {
            input! {
                p: u32, // price
                c: usize,
                f: [u32; c], // functions
            }
            (p, f)
        })
        .collect();
    let ans = (0..n).permutations(2).any(|v| {
        let i = v[0];
        let j = v[1];
        let (p_i, f_i) = &items[i];
        let (p_j, f_j) = &items[j];
        p_i >= p_j && f_i.iter().all(|f| f_j.contains(f)) && (p_i > p_j || f_j.len() > f_i.len())
    });
    println!("{}", if ans { "Yes" } else { "No" });
}
