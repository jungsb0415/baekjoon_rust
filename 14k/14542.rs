use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let mut c = 0;
    loop {
        let n: usize = input.next().unwrap().parse().unwrap();
        if n == 0 {
            break;
        }
        c += 1;

        let mut ans = 0;
        for i in 1..=n {
            for j in 1..=i {
                let k: usize = input.next().unwrap().parse().unwrap();

                if j == 1 || i == j || i == n {
                    ans += k;
                }
            }
        }

        write!(output, "Case #{c}:{}\n", ans).unwrap();
    }

    // write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}
