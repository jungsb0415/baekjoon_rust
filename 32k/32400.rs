use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let mut v: Vec<i32> = vec![];
    let mut idx = 0usize;
    for i in 0..20 {
        let k: i32 = input.next().unwrap().parse().unwrap();
        if k == 20 {
            idx = i;
        }
        v.push(k);
    }

    let a = v[(idx + 19) % 20] + v[idx] + v[(idx + 1) % 20];

    let ans = if a >= 32 { "Alice" } else { "Bob" };

    write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}
