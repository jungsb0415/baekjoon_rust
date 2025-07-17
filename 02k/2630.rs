use std::fmt::Write;
use std::io::{Read, stdin};

fn f(paper: &Vec<Vec<i32>>, row: usize, col: usize,
    n: i32, white: &mut i32, blue: &mut i32) {
    if n == 1 {
        if paper[row][col] == 0 {
            *white += 1;
        } else {
            *blue += 1;
        }
        return;
    }

    let mut sum = 0;

    for i in 0..n as usize {
        for j in 0..n as usize {
            sum += paper[row + i][col + j];
        }
    }

    if sum == 0 {
        *white += 1;
        return;
    }

    if sum == n*n {
        *blue += 1;
        return;
    }
    let half = n / 2;
    let half_usize = half as usize;

    f(paper, row, col, half, white, blue);
    f(paper, row + half_usize, col, half, white, blue);
    f(paper, row, col + half_usize, half, white, blue);
    f(paper, row + half_usize, col + half_usize, half, white, blue);
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n = next_i32(&mut input);

    let paper: Vec<Vec<i32>> = (0..n)
        .map(|_| (0..n).map(|_| next_i32(&mut input)).collect()).collect();

    let mut white = 0;
    let mut blue = 0;

    f(&paper, 0, 0, n, &mut white, &mut blue);
    write!(output, "{}\n{}\n", white, blue).unwrap();

    // write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}

fn next_u32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> u32 {
    input.next().unwrap().parse().unwrap()
}

fn next_i32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> i32 {
    input.next().unwrap().parse().unwrap()
}
