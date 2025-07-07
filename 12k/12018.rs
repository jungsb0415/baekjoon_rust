use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n: usize = input.next().unwrap().parse().unwrap();
    let m: usize = input.next().unwrap().parse().unwrap();

    let mut mile: Vec<usize> = vec![];

    for _ in 0..n {
        let p: usize = input.next().unwrap().parse().unwrap();
        let l: usize = input.next().unwrap().parse().unwrap();
        let mut v: Vec<usize> = (0..p)
            .map(|_| input.next().unwrap().parse().unwrap())
            .collect();

        v.sort_by(|a, b| b.cmp(a));

        if p < l {
            mile.push(1);
        } else {
            mile.push(v[l - 1])
        }
    }

    mile.sort();

    let mut sum = 0;
    let mut cnt = 0;

    for i in mile {
        if sum + i > m {
            break;
        }
        sum += i;
        cnt += 1;
    }

    write!(output, "{}\n", cnt).unwrap();
    print!("{output}");
}
