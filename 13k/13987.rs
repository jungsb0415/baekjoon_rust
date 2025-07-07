use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n: usize = 6;

    let v: Vec<usize> = (0..n)
        .map(|_| input.next().unwrap().parse().unwrap())
        .collect();
    let u: Vec<usize> = (0..n)
        .map(|_| input.next().unwrap().parse().unwrap())
        .collect();

    let mut cnt = 0;
    let mut tot = 0;

    for i in 0..n {
        for j in 0..n {
            if v[i] != u[j] {
                tot += 1;
                if v[i] > u[j] {
                    cnt += 1;
                }
            }
        }
    }

    let ans = cnt as f64 / tot as f64;
    let ans = (ans * 100000.0).round() / 100000.0;

    write!(output, "{:.5}\n", ans).unwrap();
    print!("{output}");
}
