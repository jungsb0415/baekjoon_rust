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
        let v: Vec<usize> = (0..n)
            .map(|_| input.next().unwrap().parse().unwrap())
            .collect();

        let sum: usize = v.iter().sum();
        let cnt = sum - n;

        let mut ans: i32 = 0;

        if cnt == 0 {
            ans = 1;
        } else if cnt % 2 == 1 {
            ans = -1;
        } else {
            let mut a = 0;
            for i in 0..n {
                if v[i] == 2 {
                    a += 1;
                    if cnt == a * 2 {
                        ans = 1 + i as i32;
                        break;
                    }
                }
            }
        }
        write!(output, "{}\n", ans).unwrap();
    }

    print!("{output}");
}
