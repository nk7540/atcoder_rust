use std::collections::VecDeque;

use proconio::{input, marker::Chars};
#[allow(clippy::uninlined_format_args)]
// Unsupported features in 1.42.0
// usize::MAX, u32::MAX etc. (use usize::max_value() instead)
// from_digit
// the trait `std::convert::From<[(usize, usize); 1]>` is not implemented for `std::collections::VecDeque<_>`
// the trait `std::iter::Iterator` is not implemented for `[({integer}, {integer}); 4]`

const INF: usize = usize::max_value();

fn main() {
    input! {
        row: usize,
        col: usize,
        sx: usize,
        sy: usize,
        gx: usize,
        gy: usize,
        grid: [Chars; row],
    }
    let sx = sx - 1;
    let sy = sy - 1;
    let gx = gx - 1;
    let gy = gy - 1;

    let dxy = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut deque = VecDeque::new();
    deque.push_back((sx, sy));
    let mut dist = vec![vec![INF; col]; row];
    dist[sx][sy] = 0;

    while let Some((x, y)) = deque.pop_front() {
        for &(dx, dy) in &dxy {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if (0..row).contains(&nx)
                && (0..col).contains(&ny)
                && dist[nx][ny] == INF
                && grid[nx][ny] == '.'
            {
                deque.push_back((nx, ny));
                dist[nx][ny] = dist[x][y] + 1;
            }
        }
    }
    println!("{}", dist[gx][gy]);
}
