use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n: usize = input.next().unwrap().parse().unwrap();

    let mut ans = 0;
    let mut cur = 0;

    for _ in 0..n {
        let k: usize = input.next().unwrap().parse().unwrap();

        if cur + 1 <= k {
            cur += 1;
        } else {
            ans = ans.max(cur);
            cur = k;
        }
    }

    ans = ans.max(cur);

    write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}
