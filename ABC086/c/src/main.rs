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
    let values: Vec<(i32, i32, i32,)> = (0..n).map(|_| (read(), read(), read())).collect();

    let mut results = Vec::with_capacity(n as usize);
    for v in values.iter() {
        let t = v.0;
        let sum = v.1.abs() + v.2.abs();
        if t < sum {
            results.push(1)
        }
        if t % 2 == 1 && sum % 2 == 0 {
            results.push(1)
        }
        if t % 2 == 0 && sum % 2 == 1 {
            results.push(1)
        }
        results.push(0)
    }

    if results.iter().all(|&v| v == 0) {
        println!("Yes")
    } else {
        println!("No")
    }
}
