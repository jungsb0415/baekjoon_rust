use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n: u64 = input.next().unwrap().parse().unwrap();
    let mut ans = 0;

    let mut tmp: u64 = 2;
    loop {
        for j in 0..100 {
            let s = tmp * 3u64.pow(j);
            if s > n {
                break;
            }
            ans += 1;
        }

        tmp *= 2;
        if tmp > n {
            break;
        }
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
