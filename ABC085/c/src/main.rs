use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main() {
    let n: i32 = read();
    let amount: i32 = read();
    // i + j + k = n
    // 10000i + 5000j + 1000k = amount
    // 1 <= n <= 2000
    // 1000 <= amount <= 2 * 10^7
    let mut ans = (-1, -1, -1);

    'out: for i in 0..(n + 1) {
        for j in 0..(n - i + 1) {
            let k = n - i - j;
            if 10000 * i + 5000 * j + 1000 * k == amount {
                ans = (i as i32, j as i32, k as i32);
                break 'out;
            }

        }
    }

    println!("{} {} {}", ans.0, ans.1, ans.2);

}
