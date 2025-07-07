use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n: u64 = input.next().unwrap().parse().unwrap();
    let h: u64 = input.next().unwrap().parse().unwrap();

    let mut p = true;
    let mut ans = 0;

    for _ in 0..n {
        let a: u64 = input.next().unwrap().parse().unwrap();
        let b: u64 = input.next().unwrap().parse().unwrap();
        let c: u64 = input.next().unwrap().parse().unwrap();

        let mut v = vec![a, b, c];
        v.sort();

        if v[0] > h {
            p = false;
        } else if v[1] > h {
            ans += v[1];
        } else {
            ans += v[0];
        }
    }

    if p {
        write!(output, "{}\n", ans).unwrap();
    } else {
        write!(output, "impossible\n").unwrap();
    }

    // write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}
