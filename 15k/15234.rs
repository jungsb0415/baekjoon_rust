use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n: usize = input.next().unwrap().parse().unwrap();
    let k: usize = input.next().unwrap().parse().unwrap();

    let mut v: Vec<usize> = (0..n)
        .map(|_| input.next().unwrap().parse().unwrap())
        .collect();

    v.sort();

    let mut i = 0usize;
    let mut j = n - 1;

    let mut ans = 0;

    while i < j {
        let sum = v[i] + v[j];

        if sum == k {
            ans += 1;
            i += 1;
            j -= 1;
        } else if sum < k {
            i += 1;
        } else {
            j -= 1;
        }
    }

    write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}
