use proconio::input;
#[allow(clippy::uninlined_format_args)]

fn main() {
    input! {
        n: usize,
        mut v: [[i32; 3]; n],
    }
    v.insert(0, vec![0; 3]);
    let mut res = true;
    for i in 0..n {
        let dt = v[i + 1][0] - v[i][0];
        let dx_abs = (v[i + 1][1] - v[i][1]).abs();
        let dy_abs = (v[i + 1][2] - v[i][2]).abs();
        let dist = dx_abs + dy_abs;

        if !(dist <= dt && (dt - dist) % 2 == 0) {
            res = false;
            break;
        }
    }
    println!("{}", if res { "Yes" } else { "No" });
}
