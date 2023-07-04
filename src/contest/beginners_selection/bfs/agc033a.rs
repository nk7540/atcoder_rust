use std::collections::VecDeque;

use proconio::{input, marker::Chars};
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`

const INF: usize = std::usize::MAX;

fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h],
    }
    let mut deque = VecDeque::new();
    let mut dist = vec![vec![INF; w]; h];
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '#' {
                deque.push_back((i, j));
                dist[i][j] = 0;
            }
        }
    }
    const DXY: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut ans = 0;
    while let Some((x, y)) = deque.pop_front() {
        for &(dx, dy) in DXY.iter() {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if (0..h).contains(&nx)
                && (0..w).contains(&ny)
                && dist[nx][ny] == INF
                && grid[nx][ny] == '.'
            {
                dist[nx][ny] = dist[x][y] + 1;
                deque.push_back((nx, ny));
                ans = dist[nx][ny];
            }
        }
    }
    println!("{}", ans);
}
