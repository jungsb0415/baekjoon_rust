use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n: usize = input.next().unwrap().parse().unwrap();

    let v: Vec<i32> = (0..n)
        .map(|_| input.next().unwrap().parse().unwrap())
        .collect();
    let u: Vec<i32> = (0..n)
        .map(|_| input.next().unwrap().parse().unwrap())
        .collect();

    let mut ans = 0;
    let mut a = 0;
    let mut b = 0;

    for i in 0..n {
        a += v[i];
        b += u[i];

        if a == b {ans = i+1;}
    }
    
    write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}
