#![feature(test)]
extern crate cli_test_dir;
extern crate test;
use std::collections::VecDeque;

use proconio::input;
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`
// no method named `abs_diff` found for type `u32`

/// s: start
/// n: # of nodes
/// returns the largest shortest path length from s and its node
fn bfs(s: usize, n: usize, edges: &Vec<(usize, usize)>) -> u32 {
    let mut deque = VecDeque::new();
    let mut dist = vec![std::u32::MAX; n];
    deque.push_back(s);
    dist[s - 1] = 0;
    let mut max_len = 0;
    while let Some(node) = deque.pop_front() {
        for &(es, et) in edges.iter() {
            let visiting = edges
            if dist[visiting - 1] == std::u32::MAX {
                dist[visiting - 1] = dist[node - 1] + 1;
                max_len = dist[visiting - 1];
                deque.push_back(visiting);
            }
        }
    }
    max_len
}

fn main() {
    input! {
        n1: usize,
        n2: usize,
        m: usize,
        edges: [(usize, usize); m],
    }
    let edges = edges
        .iter()
        .flat_map(|&(s, t)| vec![(s, t), (t, s)])
        .collect();
    let n = n1 + n2;
    let d1 = bfs(1, n, &edges);
    let d2 = bfs(n, n, &edges);
    println!("{}", d1 + d2 + 1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use cli_test_dir::{CommandExt, ExpectStatus, OutputExt, TestDir};
    use rand::Rng;
    use test::Bencher;

    #[test]
    fn test_solve() {
        let cases = [
            (
                "3 4 6
1 2
2 3
4 5
4 6
1 3
6 7",
                "5",
            ),
            (
                "7 5 20
10 11
4 5
10 12
1 2
1 5
5 6
2 4
3 5
9 10
2 5
1 4
11 12
9 12
8 9
5 7
3 7
3 6
3 4
8 12
9 11",
                "4",
            ),
        ];

        let testdir = TestDir::new("abc309_d", "");
        for (i, (stdin, stdout)) in cases.iter().enumerate() {
            let output = testdir.cmd().output_with_stdin(stdin).expect_success();
            assert_eq!(
                output.stdout_str(),
                format!("{}\n", stdout),
                "test case {}",
                i + 1
            );
            assert!(output.stderr_str().is_empty(), "test case {}", i + 1);
        }
    }

    #[bench]
    fn bench_solve(b: &mut Bencher) {}
}
