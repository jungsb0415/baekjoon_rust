use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n = next_u32(&mut input);

    let mut cnt = 0;
    let mut max = 0;
    let mut cur_len = 1;

    let mut prev = next_u32(&mut input);
    let mut next = 0;

    for _ in 1..n {
        next = next_u32(&mut input);
        if prev > next {
            cnt += 1;
            max = max.max(cur_len);
            cur_len = 1;
        } else {
            cur_len += 1;
        }

        prev = next;
    }

    max = max.max(cur_len);
    cnt += 1;

    write!(output, "{} {}\n", cnt, max).unwrap();
    print!("{output}");
}

fn next_u32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> u32 {
    input.next().unwrap().parse().unwrap()
}

fn next_i32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> i32 {
    input.next().unwrap().parse().unwrap()
}