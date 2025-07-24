use std::collections::HashMap;
use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n = next_i32(&mut input);
    let mut hash: HashMap<char, &str> = HashMap::new();

    for _ in 0..n {
        let str = input.next().unwrap();
        let ch: char = input.next().unwrap().parse().unwrap();

        hash.insert(ch, str);
    }

    let compressed = input.next().unwrap();
    let front = next_i32(&mut input) as usize;
    let end = next_i32(&mut input) as usize;

    let mut str: String = String::new();

    for ch in compressed.chars() {
        str.push_str(hash.get(&ch).unwrap());
    }

    let ans = &str[front-1..end];

    write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}

fn next_i32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> i32 {
    input.next().unwrap().parse().unwrap()
}
