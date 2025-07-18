use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let t = next_i32(&mut input);

    for _ in 0..t {
        let s = next_i32(&mut input);
        let n = next_i32(&mut input);
        let f = next_i32(&mut input);
        let m = next_i32(&mut input);

        let ans = if n + m <= s && s <= n * f + m {
            "YES"
        } else {
            "NO"
        };
        write!(output, "{}\n", ans).unwrap();

    }

    // write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}

fn next_u32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> u32 {
    input.next().unwrap().parse().unwrap()
}

fn next_i32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> i32 {
    input.next().unwrap().parse().unwrap()
}
