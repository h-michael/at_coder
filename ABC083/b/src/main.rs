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
    let a: u32 = read();
    let b: u32 = read();

    let mut sum = 0;
    for num in 0..n + 1 {
        if compare(sum2(num.to_string()), a, b) {
            sum = sum + num;
        }
    }
    println!("{}", sum);
}

fn sum2(num_string: String) -> u32 {
    num_string.chars().map(|c| c.to_string().parse::<u32>().unwrap()).fold(0, |acc, x| acc + x)
}

fn compare(x: u32, a: u32, b: u32) -> bool {
    if a <= x {
        if b >= x {
            return true
        }
    }
    false
}
