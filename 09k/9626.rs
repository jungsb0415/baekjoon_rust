use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n = next_u32(&mut input);
    let m = next_u32(&mut input);
    let u = next_u32(&mut input);
    let l = next_u32(&mut input);
    let r = next_u32(&mut input);
    let d = next_u32(&mut input);

    let v: Vec<Vec<char>> = (0..n)
        .map(|_| input.next().unwrap().chars().collect()).collect();

    for i in 0..n+u+d {
        for j in 0..m+l+r {
            let c = if u <= i && i < n+u && l <= j && j < m+l {
                v[i as usize - u as usize][j as usize - l as usize]
            } else if (i+j)%2 == 0 {
                '#'
            } else {
                '.'
            };
            write!(output, "{}", c).unwrap();
        }
        write!(output, "\n").unwrap();
    }

    // write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}

fn next_u32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> u32 {
    input.next().unwrap().parse().unwrap()
}

fn next_i32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> i32 {
    input.next().unwrap().parse().unwrap()
}
