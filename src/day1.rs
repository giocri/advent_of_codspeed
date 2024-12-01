use std::fmt::Display;

use wide::{i32x4, i32x8, CmpGt, CmpLt};

pub fn part1(input: &str) -> impl Display {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    vec1.reserve(10000);
    vec2.reserve(10000);
    for line in input.lines() {
        let mut row = line.split_ascii_whitespace();
        vec1.push(row.next().unwrap().parse::<i32>().unwrap());
        vec2.push(row.next().unwrap().parse::<i32>().unwrap());
    }
    vec1.sort();
    vec2.sort();
    let mut out = 0;
    for i in 0..vec1.len() {
        out += (vec1[i] - vec2[i]).abs();
    }
    out
}
pub fn part2(input: &str) -> impl Display {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    vec1.reserve(10000);
    vec2.reserve(10000);
    for line in input.lines() {
        let mut row = line.split_ascii_whitespace();
        vec1.push(row.next().unwrap().parse::<i32>().unwrap());
        vec2.push(row.next().unwrap().parse::<i32>().unwrap());
    }
    vec1.sort();
    vec2.sort();
    let mut out = 0;
    let mut progress = 0;
    for left in vec1 {
        let check: i32x8 = left.into();
        'prefix: {
            while progress < vec2.len() - 7 {
                let block = i32x8::new([
                    vec2[progress],
                    vec2[progress + 1],
                    vec2[progress + 2],
                    vec2[progress + 3],
                    vec2[progress + 4],
                    vec2[progress + 5],
                    vec2[progress + 6],
                    vec2[progress + 7],
                ]);
                let count = check.cmp_gt(block).reduce_add();
                progress -= count as usize; //sign is inverted
                if count > -8 {
                    break 'prefix;
                }
            }
            while vec2[progress] < left {
                progress += 1;
            }
        }
        'count: {
            while progress < vec2.len() - 7 {
                let block = i32x8::new([
                    vec2[progress],
                    vec2[progress + 1],
                    vec2[progress + 2],
                    vec2[progress + 3],
                    vec2[progress + 4],
                    vec2[progress + 5],
                    vec2[progress + 6],
                    vec2[progress + 7],
                ]);
                //let check: i32x4 = left.into();
                let count = 4 + check.cmp_lt(block).reduce_add();
                progress += count as usize;
                out += count * left;
                if count < 8 {
                    break 'count;
                }
            }
            while vec2[progress] < left {
                progress += 1;
                out += left;
            }
        }
    }
    out
}
