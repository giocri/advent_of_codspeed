use count_digits::CountDigits;
use std::{collections::HashMap, fmt::Display};
enum NextMove {
    SINGLE(u64),
    DOUBLE(u64, u64),
    UNCOMPUTED,
}
pub fn part1(input: &str) -> impl Display {
    let mut stones: HashMap<u64, (NextMove, u64)> = HashMap::new();
    stones.insert(0, (NextMove::SINGLE(1), 0));
    let mut generated: Vec<(u64, u64)> = Vec::new();
    for n in input.split_ascii_whitespace() {
        let num = u64::from_str_radix(n, 10).unwrap();
        if let Some((_next, amount)) = stones.get_mut(&num) {
            *amount += 1;
        } else {
            if n.len() % 2 == 0 {
                let first = u64::from_str_radix(&n[0..n.len() / 2], 10).unwrap();
                let second = u64::from_str_radix(&n[n.len() / 2..], 10).unwrap();
                stones.insert(num, (NextMove::DOUBLE(first, second), 1));
            } else {
                let next = num * 2024;
                stones.insert(num, (NextMove::SINGLE(next), 1));
            }
        }
    }
    for _ in 0..25 {
        generated.clear();
        for (engraving, (next_move, amount)) in stones.iter_mut() {
            if *amount == 0 {
                continue;
            }
            match next_move {
                NextMove::SINGLE(x) => generated.push((*x, *amount)),
                NextMove::DOUBLE(x, y) => {
                    generated.push((*x, *amount));
                    generated.push((*y, *amount))
                }
                NextMove::UNCOMPUTED => {
                    let log10: u32 = u64::count_digits(*engraving).try_into().unwrap();
                    if log10 % 2 == 0 {
                        let split = 10u64.pow(log10 / 2);
                        let x = engraving / split;
                        let y = engraving % split;
                        *next_move = NextMove::DOUBLE(x, y);
                        generated.push((x, *amount));
                        generated.push((y, *amount))
                    } else {
                        let x = engraving * 2024;
                        *next_move = NextMove::SINGLE(x);
                        generated.push((x, *amount));
                    }
                }
            }
            *amount = 0;
        }
        for (x, new_amount) in generated.iter() {
            if let Some((_, amount)) = stones.get_mut(&x) {
                *amount += new_amount;
            } else {
                stones.insert(*x, (NextMove::UNCOMPUTED, *new_amount));
            }
        }
    }
    stones.iter().fold(0, |acc, (_, (_, x))| acc + x)
}
pub fn part2(input: &str) -> impl Display {
    let mut stones: HashMap<u64, (NextMove, u64)> = HashMap::new();
    stones.insert(0, (NextMove::SINGLE(1), 0));
    let mut generated: Vec<(u64, u64)> = Vec::new();
    for n in input.split_ascii_whitespace() {
        let num = u64::from_str_radix(n, 10).unwrap();
        if let Some((_next, amount)) = stones.get_mut(&num) {
            *amount += 1;
        } else {
            if n.len() % 2 == 0 {
                let first = u64::from_str_radix(&n[0..n.len() / 2], 10).unwrap();
                let second = u64::from_str_radix(&n[n.len() / 2..], 10).unwrap();
                stones.insert(num, (NextMove::DOUBLE(first, second), 1));
            } else {
                let next = num * 2024;
                stones.insert(num, (NextMove::SINGLE(next), 1));
            }
        }
    }
    for _ in 0..75 {
        generated.clear();
        for (engraving, (next_move, amount)) in stones.iter_mut() {
            if *amount == 0 {
                continue;
            }
            match next_move {
                NextMove::SINGLE(x) => generated.push((*x, *amount)),
                NextMove::DOUBLE(x, y) => {
                    generated.push((*x, *amount));
                    generated.push((*y, *amount))
                }
                NextMove::UNCOMPUTED => {
                    let log10: u32 = u64::count_digits(*engraving).try_into().unwrap();
                    if log10 % 2 == 0 {
                        let split = 10u64.pow(log10 / 2);
                        let x = engraving / split;
                        let y = engraving % split;
                        *next_move = NextMove::DOUBLE(x, y);
                        generated.push((x, *amount));
                        generated.push((y, *amount))
                    } else {
                        let x = engraving * 2024;
                        *next_move = NextMove::SINGLE(x);
                        generated.push((x, *amount));
                    }
                }
            }
            *amount = 0;
        }
        for (x, new_amount) in generated.iter() {
            if let Some((_, amount)) = stones.get_mut(&x) {
                *amount += new_amount;
            } else {
                stones.insert(*x, (NextMove::UNCOMPUTED, *new_amount));
            }
        }
    }
    stones.iter().fold(0, |acc, (_, (_, x))| acc + x)
}
