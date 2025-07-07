use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let w: i64 = input.next().unwrap().parse().unwrap();
    let h: i64 = input.next().unwrap().parse().unwrap();
    let k: i64 = input.next().unwrap().parse().unwrap();

    let m: i64 = input.next().unwrap().parse().unwrap();
    let mut col: Vec<i64> = vec![];
    let mut prev = 0;
    for _i in 0..m {
        let num: i64 = input.next().unwrap().parse().unwrap();
        col.push(num - prev);
        prev = num;
    }
    col.push(h - prev);

    let n: i64 = input.next().unwrap().parse().unwrap();
    let mut row: Vec<i64> = vec![];
    prev = 0;
    for _i in 0..n {
        let num: i64 = input.next().unwrap().parse().unwrap();
        row.push(num - prev);
        prev = num;
    }
    row.push(w - prev);

    row.sort();
    col.sort();

    let mut ans = 0i64;

    for r in row {
        if col[0] * r > k {
            break;
        }
        let mut f = 0usize;
        let mut e = col.len();

        while f + 1 < e {
            let m = (f + e) / 2;
            if col[m] * r > k {
                e = m;
            } else {
                f = m;
            }
        }

        ans += f as i64 + 1;
    }

    write!(output, "{}\n", ans).unwrap();
    print!("{output}");
}
