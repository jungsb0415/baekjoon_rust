
use std::fmt::Write;
use std::io::{Read, stdin};

fn cal((i1, i2): (i32, i32),
       (j1, j2): (i32, i32),
       (k1, k2): (i32, i32)) -> i32 {

        if i1 == j1 && i2 == k2 {(i1*j2 + j1*k2 + k1*i2 - i2*j1 - j2*k1 - k2*i1).abs()}
        else {0}
       }

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let t: u32 = next_u32(&mut input);
    let t = t as usize;

    let mut v: Vec<(i32, i32)> = vec![];

    for _ in 0..t {
        let a = next_i32(&mut input);
        let b = next_i32(&mut input);

        v.push((a, b));
    }

    let mut ans = 0;

    for i in 0..t {
        for j in 0..t {
            for k in 0..t {
                let area = cal(v[i], v[j], v[k]);
                ans = ans.max(area);
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


