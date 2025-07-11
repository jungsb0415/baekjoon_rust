use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n: i32 = next_i32(&mut input);
    let k: i32 = next_i32(&mut input);
    let m: i32 = next_i32(&mut input);

    let v: Vec<i32> = (0..n as usize)
        .map(|_| next_i32(&mut input)).collect();

    let mut ans = k;

    for _ in 0..m {
        let oper = next_i32(&mut input);

        if oper > 0 {
            if ans <= oper {
                ans = oper - ans + 1;
            }
        } else {
            if ans >= n + oper + 1 {
                ans = n + n + oper - ans + 1;
            }
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
