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
    let mut arr = vec![];

    for _num in 0..n {
        arr.push(read::<u32>());
    }
    let arr = quick_sort(arr);
    let mut alice: u32 = 0;
    let mut bob: u32 = 0;
    for (i, num) in arr.iter().rev().enumerate() {
        if i % 2 == 0 {
            alice += num.clone();
        } else {
            bob += num.clone();
        }
    }
    println!("{:?}", alice - bob);
}

fn quick_sort(mut arr: Vec<u32>) -> Vec<u32> {
    if arr.len() <= 2 {
        let last = arr.pop();
        let first = arr.pop();
        if first <= last {
            if let Some(f) = first {
                arr.push(f);
            }
            if let Some(l) = last {
                arr.push(l);
            }
        } else {
            if let Some(l) = last {
                arr.push(l);
            }
            if let Some(f) = first {
                arr.push(f);
            }
        }
        return arr
    }
    let piv = arr.pop().unwrap();

    let mut lt = vec![];
    let mut gt = vec![];

    for num in arr {
        if num <= piv {
            lt.push(num);
        } else {
            gt.push(num);
        }
    }

    let mut lt = quick_sort(lt);
    let gt = quick_sort(gt);

    lt.push(piv);
    lt.append(&mut quick_sort(gt));
    lt
}
