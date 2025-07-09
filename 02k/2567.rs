use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n = next_u32(&mut input);

    let mut v: Vec<Vec<bool>> = vec![vec![false; 102]; 102];

    for _ in 0..n {
        let x = next_u32(&mut input);
        let y = next_u32(&mut input);

        for i in 0..10 {
            for j in 0..10 {
                v[x as usize + i][y as usize + j] = true;
            }
        }
    }

    let mut cnt = 0;

    for x in 1..=100 {
        for y in 1..=100 {
            if v[x][y] {
                let mut tmp: i32 = 0;
                if !v[x-1][y] { tmp += 1; }
                if !v[x+1][y] { tmp += 1; }
                if !v[x][y-1] { tmp += 1; }
                if !v[x][y+1] { tmp += 1; }

                cnt += tmp;
            }
        }
    }

    write!(output, "{}\n", cnt).unwrap();
    print!("{output}");
}

fn next_u32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> u32 {
    input.next().unwrap().parse().unwrap()
}

fn next_i32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> i32 {
    input.next().unwrap().parse().unwrap()
}