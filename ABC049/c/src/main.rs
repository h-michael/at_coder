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
    let words: Vec<Vec<char>> = ["dream", "dreamer", "erase", "eraser"].iter().map(|word| word.chars().rev().collect()).collect();

    let text: Vec<char> = read::<String>().chars().rev().collect();
    let mut text = &text[..];
    let mut result = true;

    while text.len() > 0 {
        let matched = words.iter().find(|word| text.starts_with(word));
        if let Some(word) = matched {
            text = &text[word.len()..];
        } else {
            result = false;
            break;
        }
    }
    println!("{}", if result { "YES" } else { "NO" });
}
