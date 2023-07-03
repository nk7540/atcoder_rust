use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }
    let mut cnt = 0;
    for na in 0..a + 1 {
        for nb in 0..b + 1 {
            for nc in 0..c + 1 {
                if 500 * na + 100 * nb + 50 * nc == x {
                    cnt += 1;
                }
            }
        }
    }
    println!("{cnt}");
}
