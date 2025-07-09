use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n = next_u32(&mut input);

    let mut v: Vec<u32> = (0..n)
        .map(|_| next_u32(&mut input)).collect();

    v.sort();

    let mut ans = 4;

    for num in &v {
        let mut tmp = 0;

        for i in 0..5 {
            if v.contains(&(num + i)) {
                tmp += 1;
            }
        }

        ans = ans.min(5 - tmp);
    }

    write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}

fn next_u32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> u32 {
    input.next().unwrap().parse().unwrap()
}

fn next_i32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> i32 {
    input.next().unwrap().parse().unwrap()
}