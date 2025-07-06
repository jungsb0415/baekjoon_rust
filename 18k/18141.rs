use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n: usize = input.next().unwrap().parse().unwrap();
    let mut v: Vec<i32> = vec![];

    for _i in 0..n {
        let k: i32 = input.next().unwrap().parse().unwrap();
        v.push(k);
    }
    let mut ans = "yes";
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if i != j && j != k && k != i {
                    if (v[i] - v[j]) % v[k] != 0 {
                        ans = "no";
                    }
                }
            }
        }
    }

    write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}
