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
    let mut array = vec![];
    for _i in 0..n {
        let num: u32 = read();
        array.push(num);
    }

    let counter = 0;

    let (_, result) = div_by_two(array, counter);
    println!("{:?}", result);
}

fn is_odd(num: u32) -> bool {
    match num % 2 {
        0 => true,
        _ => false,
    }
}

fn div_by_two(array: Vec<u32>, counter: u32) -> (Vec<u32>, u32) {
  let mut tmp_array = vec![];
  for num in array {
      if is_odd(num) {
          tmp_array.push(num / 2);
      } else {
          return (vec![], counter)
      }
  }
  div_by_two(tmp_array, counter + 1)
}
