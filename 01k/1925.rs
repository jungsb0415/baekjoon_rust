use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let x1 = next_i32(&mut input);
    let y1 = next_i32(&mut input);
    let x2 = next_i32(&mut input);
    let y2 = next_i32(&mut input);
    let x3 = next_i32(&mut input);
    let y3 = next_i32(&mut input);

    let d1 = (x2-x3).pow(2) + (y2-y3).pow(2);
    let d2 = (x3-x1).pow(2) + (y3-y1).pow(2);
    let d3 = (x1-x2).pow(2) + (y1-y2).pow(2);

    let mut d = vec![d1, d2, d3];
    d.sort();

    let d1 = d[0];
    let d2 = d[1];
    let d3 = d[2];

    let ans = if (x3 - x2) * (y2 - y1) == (x2 - x1) * (y3 - y2) {
        "X"
    } else if d1 == d2 && d2 == d3{
        "JungTriangle"
    } else if d1 + d2 == d3 { // 직각
        if d1 == d2 {
            "Jikkak2Triangle"
        } else {
            "JikkakTriangle"
        }
    } else if d1 + d2 < d3 { // 둔각
        if d1 == d2 {
            "Dunkak2Triangle"
        } else {
            "DunkakTriangle"
        }
    } else { // 예각
        if d1 == d2 || d2 == d3 {
            "Yeahkak2Triangle"
        } else {
            "YeahkakTriangle"
        }
    };

    write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}

fn next_u32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> u32 {
    input.next().unwrap().parse().unwrap()
}

fn next_i32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> i32 {
    input.next().unwrap().parse().unwrap()
}
