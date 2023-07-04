use proconio::{input, marker::Chars};
#[allow(clippy::uninlined_format_args)]

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    // extract start and goal positions
    // dfs with stack, return if the goal is reached
    let mut s = (0, 0);
    let mut g = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == 's' {
                s = (i, j);
            } else if c[i][j] == 'g' {
                g = (i, j);
            }
        }
    }
    let dxy = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut stack = vec![s];
    let mut visited = vec![vec![false; w]; h];
    while let Some((x, y)) = stack.pop() {
        for (dx, dy) in dxy {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if !(nx >= 0 && nx < h as i32 && ny >= 0 && ny < w as i32) {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if !visited[nx][ny] && c[nx][ny] != '#' {
                stack.push((nx, ny));
                visited[nx][ny] = true;
                if (nx, ny) == g {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
