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
    let n: u32 = read();
    let m: u32 = read();
    let l: u32 = read();
    let x: u32 = read();
    let mut result = 0;

    for i in 0..n + 1 {
        for j in 0..m + 1 {
            for k in 0..l + 1 {
                if 500 * i + 100 * j + 50 * k == x {
                    result += 1;
                }
            }
        }
    }
    println!("{}", result);
}
