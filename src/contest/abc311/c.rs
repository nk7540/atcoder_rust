use std::ops::Index;

use itertools::Itertools;
use proconio::input;
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`
// no method named `abs_diff` found for type `u32`

fn main() {
    input! {
        n: usize, // 10^5
        a: [usize; n],
    }
    let mut visited = vec![false; n];
    let mut visited_order = vec![0];
    let mut last_visited = 0;
    loop {
        last_visited = a[last_visited] - 1;

        if last_visited == 0 {
            break;
        } else if visited[last_visited] {
            let offset = visited_order
                .iter()
                .position(|&v| v == last_visited)
                .unwrap();
            visited_order = visited_order[offset..].to_vec();
            break;
        } else {
            visited_order.push(last_visited);
            visited[last_visited] = true;
        }
    }
    println!("{}", visited_order.len());
    println!("{}", visited_order.iter().map(|v| v + 1).format(" "));
}
