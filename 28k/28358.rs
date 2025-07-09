use std::fmt::Write;
use std::io::{Read, stdin};

fn f(m: i32, d: i32) -> Vec<i32> {
    let mut u: Vec<i32> = vec![0; 10];

    u[(m%10) as usize] = 1;
    u[(d%10) as usize] = 1;

    if m > 9 {
        u[(m/10) as usize] = 1;
    }
    if d > 9 {
        u[(d/10) as usize] = 1;
    }

    u
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n = next_i32(&mut input);

    for _ in 0..n {
        let v: Vec<i32> = (0..10)
            .map(|_| next_i32(&mut input)).collect();

        let mut ans = 0;
        for m in 1..=12 {
            for d in 1..=31 {
                if m==2 {
                    if d == 30 {break;}
                } else if m==4||m==6||m==9||m==11 {
                    if d == 31 {break;}
                }
                let u = f(m, d);

                let mut p = true;

                for i in 0..10 {
                    if v[i] == 1 && u[i] == 1 {
                        p = false;
                    }
                }

                if p {
                    ans += 1;
                }
            }
        }
        write!(output, "{}\n", ans).unwrap();
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