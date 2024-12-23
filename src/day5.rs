use std::{cmp, fmt::Display};
//const CONSTRAINTS: i32 = 1176;
//const UPDATES: i32 = 1377 - 1177;
pub fn part2(input: &str) -> impl Display {
    challenge(input, true)
}
pub fn part1(input: &str) -> impl Display {
    challenge(input, false)
}

fn challenge(input: &str, set: bool) -> impl Display {
    let mut constraints_map = [false; 10000];
    let mut lines = input.lines();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let p1 = u32::from_str_radix(line[0..2].as_ref(), 10).unwrap();
        let p2 = u32::from_str_radix(line[3..5].as_ref(), 10).unwrap();
        constraints_map[(p1 * 100 + p2) as usize] = true;
    }
    let mut out = 0;
    for line in lines {
        let mut pages: Vec<u32> = line
            .split(',')
            .map(|f| u32::from_str_radix(f, 10).unwrap())
            .collect();
        let old = pages.clone();
        pages.sort_by(|&a, &b| {
            if constraints_map[(a * 100 + b) as usize] {
                cmp::Ordering::Less
            } else if constraints_map[(b * 100 + a) as usize] {
                cmp::Ordering::Greater
            } else {
                cmp::Ordering::Equal
            }
        });
        if set ^ (pages == old) {
            out += pages[pages.len() / 2];
        }
    }
    out
}
