use std::collections::HashMap;
use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
    let mut output = String::new();

    let n: u32 = next_u32(&mut input);

    let mut a: HashMap<u32, u32> = HashMap::new();
    let mut b: HashMap<u32, u32> = HashMap::new();

    for _ in 0..n {
        let mut k: u32 = next_u32(&mut input);

        let mut p = 2;

        while p * p <= k {
            let mut cnt = 0;
            while k % p == 0 {
                cnt += 1;
                k /= p;
            }

            if cnt > 0 {
                *a.entry(p).or_insert(0) += cnt;
            }

            p += 1;
        }

        if k > 1 {
            *a.entry(k).or_insert(0) += 1;
        }
    }

    let m: usize = input.next().unwrap().parse().unwrap();

    for _ in 0..m {
        let mut k: u32 = next_u32(&mut input);

        let mut p = 2;

        while p * p <= k {
            let mut cnt = 0;
            while k % p == 0 {
                cnt += 1;
                k /= p;
            }

            if cnt > 0 {
                *b.entry(p).or_insert(0) += cnt;
            }

            p += 1;
        }

        if k > 1 {
            *b.entry(k).or_insert(0) += 1;
        }
    }

    let mut ans: u64 = 1;
    let modd = 1_000_000_000;
    let mut did_modd = false;

    let mut primes: Vec<u32> = a.keys().chain(b.keys()).cloned().collect();
    primes.sort_unstable();
    primes.dedup();

    for p in primes {
        let a_cnt = a.get(&p).unwrap_or(&0);
        let b_cnt = b.get(&p).unwrap_or(&0);
        let cnt = *a_cnt.min(b_cnt);

        for _ in 0..cnt {
            ans *= p as u64;
            if ans >= modd {
                ans %= modd;
                did_modd = true;
            }
        }
    }

    if did_modd {
        write!(output, "{:09}\n", ans).unwrap();
    } else {
        write!(output, "{}\n", ans).unwrap();
    }
    print!("{output}");
}

fn next_u32<'a, I: Iterator<Item = &'a str>>(input: &mut I) -> u32 {
    input.next().unwrap().parse().unwrap()
}