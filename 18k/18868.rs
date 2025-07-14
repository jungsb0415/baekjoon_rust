use std::fmt::Write;
use std::io::{Read, stdin};

fn cmp(a: i32, b: i32) -> i32 {
    if a < b {
        -1
    } else if a == b {
        0
    } else {
        1
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let m = next_i32(&mut input) as usize;
    let n = next_i32(&mut input) as usize;

    let mut v: Vec<Vec<i32>> = vec![vec![]; m];
    let mut ans = 0;

    for i in 0..m {
        for _j in 0..n {
            let k = next_i32(&mut input);
            v[i as usize].push(k)
        }
    }

    for space1 in 0..m {
        for space2 in space1+1..m {
            let mut flag = true;

            for i in 0..n {
                for j in i..n {
                    if cmp(v[space1][i], v[space1][j]) != cmp(v[space2][i], v[space2][j]) {
                        flag = false;
                        break;
                    }
                }
            }

            if flag {ans += 1;}
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
