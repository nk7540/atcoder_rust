use std::collections::VecDeque;

use proconio::{input, marker::Chars};
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`

const INF: usize = std::usize::MAX;
const dxy: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize, // # of cheeses
        grid: [Chars; h],
    }
    let mut deque = VecDeque::new();
    let mut dist = vec![vec![INF; w]; h];
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == 'S' {
                deque.push_back((i, j));
                dist[i][j] = 0;
            }
        }
    }
    // loop for n
    let mut ans = 0;
    for goal in 1..n + 1 {
        let goal = std::char::from_digit(goal as u32, 10).unwrap();
        // bfs until finding next cheese
        'bfs: while let Some((x, y)) = deque.pop_front() {
            for &(dx, dy) in dxy.iter() {
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                if (0..h).contains(&nx)
                    && (0..w).contains(&ny)
                    && dist[nx][ny] == INF
                    && grid[nx][ny] != 'X'
                {
                    if grid[nx][ny] == goal {
                        ans += dist[x][y] + 1;
                        deque = VecDeque::new();
                        dist = vec![vec![INF; w]; h];
                        deque.push_back((nx, ny));
                        dist[nx][ny] = 0;
                        break 'bfs;
                    }
                    deque.push_back((nx, ny));
                    dist[nx][ny] = dist[x][y] + 1;
                }
            }
        }
    }
    println!("{}", ans);
}
