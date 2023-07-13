use std::{collections::VecDeque, ops::Index};

use petgraph::{graph::NodeIndex, visit::Bfs, Graph};
use proconio::input;
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`
// no method named `abs_diff` found for type `u32`

/// s: start
/// n: # of nodes
/// returns the largest shortest path length from s and its node
fn bfs(s: usize, n: usize, adj_list: &[Vec<usize>]) -> u32 {
    let mut deque = VecDeque::new();
    let mut dist = vec![std::u32::MAX; n];
    deque.push_back(s);
    dist[s] = 0;
    let mut max_len = 0;
    while let Some(node) = deque.pop_front() {
        for &adj in &adj_list[node] {
            if dist[adj] == std::u32::MAX {
                dist[adj] = dist[node] + 1;
                deque.push_back(adj);
                max_len = dist[adj];
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
    let n = n1 + n2;
    let adj_list = edges
        .iter()
        .fold(vec![Vec::with_capacity(n); n], |mut acc, &(s, t)| {
            let s = s - 1;
            let t = t - 1;
            acc[s].push(t);
            acc[t].push(s);
            acc
        });
    let d1 = bfs(0, n, &adj_list);
    let d2 = bfs(n - 1, n, &adj_list);

    println!("{}", d1 + d2 + 1);
}
