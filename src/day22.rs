use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn part1(input: &str) -> u64 {
    let mut out = 0;
    let lines = input
        .lines()
        .map(|x| u32::from_str_radix(x, 10).unwrap())
        .collect_vec();
    let iter = lines.chunks_exact(16);
    for c in iter.clone() {
        let mut s1 = c[0];
        let mut s2 = c[1];
        let mut s3 = c[2];
        let mut s4 = c[3];
        let mut s5 = c[4];
        let mut s6 = c[5];
        let mut s7 = c[6];
        let mut s8 = c[7];
        let mut s9 = c[8];
        let mut s10 = c[9];
        let mut s11 = c[10];
        let mut s12 = c[11];
        let mut s13 = c[12];
        let mut s14 = c[13];
        let mut s15 = c[14];
        let mut s16 = c[15];
        for _ in 0..2000 {
            s1 = next_secret(s1);
            s2 = next_secret(s2);
            s3 = next_secret(s3);
            s4 = next_secret(s4);
            s5 = next_secret(s5);
            s6 = next_secret(s6);
            s7 = next_secret(s7);
            s8 = next_secret(s8);
            s9 = next_secret(s9);
            s10 = next_secret(s10);
            s11 = next_secret(s11);
            s12 = next_secret(s12);
            s13 = next_secret(s13);
            s14 = next_secret(s14);
            s15 = next_secret(s15);
            s16 = next_secret(s16);
        }
        let acc =
            s1 + s2 + s3 + s4 + s5 + s6 + s7 + s8 + s9 + s10 + s11 + s12 + s13 + s14 + s15 + s16;
        out += acc as u64
    }
    for x in iter.remainder() {
        let mut s1 = *x;
        for _ in 0..2000 {
            s1 = next_secret(s1);
        }
        out += s1 as u64;
    }
    out
}

pub fn part2(input: &str) -> u32 {
    let lines = input.lines().map(|x| u32::from_str_radix(x, 10).unwrap());
    let mut delta_to_total_win = HashMap::new();
    for l in lines {
        let mut old_price = l % 10;
        let mut secret = l;
        let mut delta = [0, 0, 0, 0];
        let mut seen_delta = HashSet::new();
        for i in 0..4 {
            secret = next_secret(secret);
            let new_price = secret % 10;
            delta[i] = new_price as i32 - old_price as i32;
            old_price = new_price;
        }
        for i in 4..=2000 {
            let start = i % 4;
            let key = (delta[start] + 9)
                + (delta[(start + 1) % 4] + 9) * 20
                + (delta[(start + 2) % 4] + 9) * 400
                + (delta[(start + 3) % 4] + 9) * 8000;

            if !seen_delta.contains(&key) {
                seen_delta.insert(key);
                if let Some(total) = delta_to_total_win.get_mut(&key) {
                    *total += old_price;
                } else {
                    delta_to_total_win.insert(key, old_price);
                }
            }
            secret = next_secret(secret);
            let new_price = secret % 10;
            delta[start] = new_price as i32 - old_price as i32;
            old_price = new_price;
        }
    }
    *delta_to_total_win.values().max().unwrap_or(&0)
}
fn next_secret(secret: u32) -> u32 {
    let step1 = ((secret << 6) ^ secret) % 16777216;
    let step2 = ((step1 >> 5) ^ step1) % 16777216;
    let step3 = ((step2 << 11) ^ step2) % 16777216;
    step3
}
