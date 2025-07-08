use std::fmt::Write;
use std::io::{Read, stdin};

fn cnt_vowels(str: &str) -> u32 {
    let mut cnt = 0;
    for c in str.chars() {
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            cnt += 1;
        }
    }
    cnt
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let t: u32 = next_u32(&mut input);

    for _ in 0..t {
        let s = next_str(&mut input);
        write!(output, "The number of vowels in {} is {}.\n", s, cnt_vowels(s)).unwrap();
    }

    // write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}

fn next_u32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> u32 {
    input.next().unwrap().parse().unwrap()
}

fn next_str<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> &'a str {
    input.next().unwrap()
}

