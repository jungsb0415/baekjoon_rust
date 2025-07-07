use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let t: u32 = next_u32(&mut input);

    for _ in 0..t {
        let n: u32 = next_u32(&mut input);

        let mut ans = 0;

        for i in 0..=n / 3 {
            let tmp = n - i * 3;
            ans += tmp / 2 + 1;
        }

        write!(output, "{}\n", ans).unwrap();
    }

    // write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}

fn next_u32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> u32 {
    input.next().unwrap().parse().unwrap()
}
