use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let t: usize = input.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize = input.next().unwrap().parse().unwrap();
        let v: Vec<i64> = (0..n)
            .map(|_| input.next().unwrap().parse().unwrap())
            .collect();
        let mut aftermax = v.clone();

        for i in (0..n - 1).rev() {
            aftermax[i] = aftermax[i].max(aftermax[i + 1]);
        }

        let mut ans: i64 = 0;

        for i in 0..n {
            ans += aftermax[i] - v[i];
        }

        write!(output, "{}\n", ans).unwrap();
    }

    // write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}
