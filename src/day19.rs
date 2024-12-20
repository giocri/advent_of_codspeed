use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;
pub fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let towels = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|t| {
            (
                t.bytes().len(),
                t.bytes()
                    .rev()
                    .map(|b| match b {
                        b'w' => 1,
                        b'u' => 2,
                        b'b' => 3,
                        b'r' => 4,
                        b'g' => 5,
                        _ => {
                            unreachable!()
                        }
                    })
                    .fold(0u128, |acc, d| (acc << 3) + d),
            )
        })
        .collect_vec();
    lines.next();
    let mut out = 0;
    for l in lines {
        let mut a = 0;
        let mut b = 0;
        let mut l = l;
        let len = l.len();
        if l.len() > 128 / 3 {
            let remainder = &l[(128 / 3)..];
            b = remainder
                .bytes()
                .rev()
                .map(|b| match b {
                    b'w' => 1,
                    b'u' => 2,
                    b'b' => 3,
                    b'r' => 4,
                    b'g' => 5,
                    _ => {
                        unreachable!()
                    }
                })
                .fold(0u128, |acc, d| (acc << 3) + d);
            l = &l[0..(128 / 3)];
        }
        a = l
            .bytes()
            .rev()
            .map(|b| match b {
                b'w' => 1,
                b'u' => 2,
                b'b' => 3,
                b'r' => 4,
                b'g' => 5,
                _ => {
                    unreachable!()
                }
            })
            .fold(0u128, |acc, d| (acc << 3) + d);
        let mut cache = vec![0u64; len + 5];
        cache[len] = 1;
        cache[len + 1] = 1;
        cache[len + 2] = 0;
        cache[len + 3] = 0;
        cache[len + 4] = 0;
        for x in (0..len).rev() {
            let segment = if x <= 128 / 3 {
                (a / (1 << x * 3)) + ((b % (1 << x * 3)) << ((128 / 3) * 3 - (x * 3)))
            } else {
                b / (1 << (x - (128 / 3)) * 3)
            };
            for (t_len, t) in towels.iter() {
                //println!("{} {}", x, t_len);
                if segment % (1 << (*t_len * 3)) == *t {
                    cache[x] += cache[x + t_len]
                }
            }
        }
        if cache[0] > 0 {
            out += 1;
        }
    }
    out
}
pub fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let towels = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|t| {
            (
                t.bytes().len(),
                t.bytes()
                    .rev()
                    .map(|b| match b {
                        b'w' => 1,
                        b'u' => 2,
                        b'b' => 3,
                        b'r' => 4,
                        b'g' => 5,
                        _ => {
                            unreachable!()
                        }
                    })
                    .fold(0u128, |acc, d| (acc << 3) + d),
            )
        })
        .collect_vec();
    lines.next();
    let mut out = 0;
    for l in lines {
        let mut a = 0;
        let mut b = 0;
        let mut l = l;
        let len = l.len();
        if l.len() > 128 / 3 {
            let remainder = &l[(128 / 3)..];
            b = remainder
                .bytes()
                .rev()
                .map(|b| match b {
                    b'w' => 1,
                    b'u' => 2,
                    b'b' => 3,
                    b'r' => 4,
                    b'g' => 5,
                    _ => {
                        unreachable!()
                    }
                })
                .fold(0u128, |acc, d| (acc << 3) + d);
            l = &l[0..(128 / 3)];
        }
        a = l
            .bytes()
            .rev()
            .map(|b| match b {
                b'w' => 1,
                b'u' => 2,
                b'b' => 3,
                b'r' => 4,
                b'g' => 5,
                _ => {
                    unreachable!()
                }
            })
            .fold(0u128, |acc, d| (acc << 3) + d);
        let mut cache = vec![0u64; len + 5];
        cache[len] = 1;
        cache[len + 1] = 1;
        cache[len + 2] = 0;
        cache[len + 3] = 0;
        cache[len + 4] = 0;
        for x in (0..len).rev() {
            let segment = if x <= 128 / 3 {
                (a / (1 << x * 3)) + ((b % (1 << x * 3)) << ((128 / 3) * 3 - (x * 3)))
            } else {
                b / (1 << (x - (128 / 3)) * 3)
            };
            for (t_len, t) in towels.iter() {
                //println!("{} {}", x, t_len);
                if segment % (1 << (*t_len * 3)) == *t {
                    cache[x] += cache[x + t_len]
                }
            }
        }
        out += cache[0];
    }
    out
}
