use std::fmt::Write;
use std::io::{Read, stdin};

fn f(ch: char) -> char {
    match ch {
        '.' => '.',
        'O' => 'O',
        '-' => '|',
        '|' => '-',
        '/' => '\\',
        '\\' => '/',
        '^' => '<',
        '<' => 'v',
        'v' => '>',
        _ => '^',
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n = next_i32(&mut input) as usize;
    let m = next_i32(&mut input) as usize;

    let v: Vec<Vec<char>> = (0..n)
        .map(|_| input.next().unwrap().chars().collect()).collect();

    for j in 0..m {
        for i in 0..n {
            let c = f(v[i][m-j-1]);
            write!(output, "{}", c).unwrap();
        }

        write!(output, "\n").unwrap();
    }
    // write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}

fn next_i32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> i32 {
    input.next().unwrap().parse().unwrap()
}
