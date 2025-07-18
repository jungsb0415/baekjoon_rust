use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let h_lo = next_i32(&mut input);
    let h_hi = next_i32(&mut input);
    let s_lo = next_i32(&mut input);
    let s_hi = next_i32(&mut input);
    let v_lo = next_i32(&mut input);
    let v_hi = next_i32(&mut input);
    let r = next_i32(&mut input) as f64;
    let g = next_i32(&mut input) as f64;
    let b = next_i32(&mut input) as f64;

    let big_m = r.max(g).max(b);
    let m = r.min(g).min(b);

    let v = big_m;
    let s = 255.0 * (v - m) / v;
    let h = if v == r {
        60.0 * (g - b) / (v - m)
    } else if v == g {
        120.0 + 60.0 * (b - r) / (v - m)
    } else {
        240.0 + 60.0 * (r - g) / (v - m)
    };

    let h = if h < 0.0 {
        h + 360.0
    } else {
        h
    };

    let ans = if h_lo as f64 <= h && h <= h_hi as f64 &&
    s_lo as f64 <= s && s <= s_hi as f64 &&
    v_lo as f64 <= v && v <= v_hi as f64 {
        "Lumi will like it."
    } else {
        "Lumi will not like it."
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
