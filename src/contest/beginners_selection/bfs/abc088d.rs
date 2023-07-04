use std::collections::VecDeque;

use proconio::{input, marker::Chars};
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`

fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h],
    }
    let total_white: usize = grid
        .iter()
        .map(|row| row.iter().filter(|&c| *c == '.').count())
        .sum();

    const dxy: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut deque = VecDeque::new();
    let mut dist = vec![vec![std::usize::MAX; w]; h];
    deque.push_back((0, 0));
    dist[0][0] = 0;
    while let Some((x, y)) = deque.pop_front() {
        for &(dx, dy) in dxy.iter() {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if (0..h).contains(&nx)
                && (0..w).contains(&ny)
                && dist[nx][ny] == std::usize::MAX
                && grid[nx][ny] == '.'
            {
                dist[nx][ny] = dist[x][y] + 1;
                deque.push_back((nx, ny));
            }
        }
    }
    let ans = if dist[h - 1][w - 1] == std::usize::MAX {
        -1
    } else {
        (total_white - (dist[h - 1][w - 1] + 1)) as i32
    };
    println!("{}", ans);
}
