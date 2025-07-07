use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n: usize = input.next().unwrap().parse().unwrap();

    for _ in 0..n {
        let acid = input.next().unwrap();
        let _x = input.next();
        let ph: f64 = input.next().unwrap().parse().unwrap();

        let ph = if acid == "H" {
            -ph.log10()
        } else {
            14.0 + ph.log10()
        };

        let ans = (ph * 100.0).round() / 100.0;
        write!(output, "{:.2}\n", ans).unwrap();
    }

    // write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}
