use proconio::input;
#[allow(clippy::uninlined_format_args)]

fn main() {
    let dxy = [
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, 1),
        (-1, 1),
        (-1, -1),
        (1, -1),
    ];
    loop {
        input! {
            w: usize,
            h: usize,
            c: [[usize; w]; h],
        }
        // start from (0, 0)
        // dfs, keep track of visited
        let mut visited = vec![vec![false; w]; h];
        let mut ans = 0;
        for i in 0..h {
            for j in 0..w {
                if visited[i][j] || c[i][j] == 0 {
                    continue;
                }

                // dfs
                let mut stack = vec![(i, j)];
                while let Some((x, y)) = stack.pop() {
                    for (dx, dy) in dxy {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        let nx = nx as usize;
                        let ny = ny as usize;
                        if !((0..h).contains(&nx) && (0..w).contains(&ny)) {
                            continue;
                        }
                        if !visited[nx][ny] && c[nx][ny] == 1 {
                            visited[nx][ny] = true;
                            stack.push((nx, ny));
                        }
                    }
                }

                ans += 1;
            }
        }
        println!("{}", ans);
    }
}
