use std::{cmp, collections::HashMap, fmt::Display};
const CONSTRAINTS: i32 = 1176;
const UPDATES: i32 = 1377 - 1177;
pub fn part2(input: &str) -> impl Display {
    let mut constraints_map: Vec<Vec<usize>> = vec![Vec::new(); 100];
    let mut lines = input.lines();
    for _ in 0..CONSTRAINTS {
        let line = lines.next().unwrap();
        constraints_map[u32::from_str_radix(line[0..2].as_ref(), 10).unwrap() as usize]
            .push(u32::from_str_radix(line[3..5].as_ref(), 10).unwrap() as usize);
    }
    let mut out = 0;
    for i in 0..100 {
        constraints_map[i].sort();
    }
    lines.next().unwrap();
    for _ in 0..UPDATES {
        let line = lines.next().unwrap();
        let mut pages: Vec<u32> = line
            .split(',')
            .map(|f| u32::from_str_radix(f, 10).unwrap())
            .collect();
        let old = pages.clone();
        pages.sort_by(|&a, &b| {
            if constraints_map[a as usize]
                .binary_search(&(b as usize))
                .is_ok()
            {
                cmp::Ordering::Less
            } else if constraints_map[b as usize]
                .binary_search(&(a as usize))
                .is_ok()
            {
                cmp::Ordering::Greater
            } else {
                cmp::Ordering::Equal
            }
        });
        if pages != old {
            out += pages[pages.len() / 2];
        }
    }
    out
}
pub fn part1(input: &str) -> impl Display {
    let mut constraints_map: Vec<Vec<usize>> = vec![Vec::new(); 100];
    let mut lines = input.lines();
    for _ in 0..CONSTRAINTS {
        let line = lines.next().unwrap();
        constraints_map[u32::from_str_radix(line[0..2].as_ref(), 10).unwrap() as usize]
            .push(u32::from_str_radix(line[3..5].as_ref(), 10).unwrap() as usize);
    }
    for i in 0..100 {
        constraints_map[i].sort();
    }
    let mut out = 0;
    lines.next().unwrap();
    for _ in 0..UPDATES {
        let line = lines.next().unwrap();
        let mut pages: Vec<u32> = line
            .split(',')
            .map(|f| u32::from_str_radix(f, 10).unwrap())
            .collect();
        let old = pages.clone();
        pages.sort_by(|&a, &b| {
            if constraints_map[a as usize]
                .binary_search(&(b as usize))
                .is_ok()
            {
                cmp::Ordering::Less
            } else if constraints_map[b as usize]
                .binary_search(&(a as usize))
                .is_ok()
            {
                cmp::Ordering::Greater
            } else {
                cmp::Ordering::Equal
            }
        });
        if pages == old {
            out += pages[pages.len() / 2];
        }
    }
    out
}
