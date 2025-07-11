use std::fmt::Write;
use std::io::{Read, stdin};

fn check(prev: Vec<i32>, next: Vec<i32>, d: i32) -> bool {
    for i in 0..prev.len() {
        if prev[i] - d >= next[i] {
            return false;
        }
    }
    true
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n: i32 = next_i32(&mut input);
    let m: i32 = next_i32(&mut input);
    let d: i32 = next_i32(&mut input);

    let mut v: Vec<i32> = (0..m as usize).
        map(|_| next_i32(&mut input)).collect();

    v.sort();

    let mut p = true;

    for _ in 1..n {
        let mut next: Vec<i32> = (0..m as usize).
        map(|_| next_i32(&mut input)).collect();

        next.sort();

        if !check(v.clone(), next.clone(), d) {
            p = false;
        }

        v = next;
    }

    let ans = if p {"YES"} else {"NO"};

    write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}

fn next_u32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> u32 {
    input.next().unwrap().parse().unwrap()
}

fn next_i32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> i32 {
    input.next().unwrap().parse().unwrap()
}
