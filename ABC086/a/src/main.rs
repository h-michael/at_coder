use std::io::Read;

fn main() {
    let mut buf = String::new();

    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let result = match n * m % 2 {
        0 => "Even",
        1 => "Odd",
        _ => panic!(),
    };

    println!("{}", result);
}
