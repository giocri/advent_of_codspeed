use std::{fmt::Display, iter::once, slice::Iter};

use wide::{i32x4, i32x8, CmpGt, CmpLt};
fn check_row(row: &mut impl Iterator<Item = i32>) -> Option<usize> {
    let diff = row.next().unwrap();
    let sign;
    if diff == 0 || diff.abs() > 3 {
        return Some(1);
    } else {
        sign = diff.signum();
    };
    let i = 2;
    while let Some(diff) = row.next() {
        if diff * sign < 1 || diff.abs() > 3 {
            return Some(i);
        }
    }
    None
}

pub fn part1(input: &str) -> impl Display {
    let mut out = 0;
    'a: for line in input.lines() {
        let row = line.split_ascii_whitespace();
        let row: Vec<i32> = row.map(|a| a.parse::<i32>().unwrap()).collect();
        if let Some(_) = check_row(&mut row.windows(2).map(|a| (a[1] - a[0]))) {
            continue 'a;
        }
        out += 1;
    }
    out
}
pub fn part2(input: &str) -> impl Display {
    let mut out = 0;
    'a: for line in input.lines() {
        let row = line.split_ascii_whitespace();
        let row: Vec<i32> = row.map(|a| a.parse::<i32>().unwrap()).collect();
        if let Some(x) = check_row(&mut row.windows(2).map(|a| (a[1] - a[0]))) {
            let joint = row[x + 1] - row[x - 1];
            let mut v2 = row[..x]
                .windows(2)
                .map(|a| (a[1] - a[0]))
                .chain(once(joint))
                .chain(row[(x + 1)..].windows(2).map(|a| (a[1] - a[0])));
            if check_row(&mut v2) == None {
                out += 1;
                continue 'a;
            }
            if x > 0 {
                let joint = row[x + 1] - row[x - 1];
                if x > 1 {
                    let mut v3 = row[..x - 1]
                        .windows(2)
                        .map(|a| (a[1] - a[0]))
                        .chain(once(joint))
                        .chain(row[(x + 1)..].windows(2).map(|a| (a[1] - a[0])));
                    if check_row(&mut v3) == None {
                        out += 1;
                        continue 'a;
                    }
                } else {
                    let mut v3 = row[(x + 1)..].windows(2).map(|a| (a[1] - a[0]));
                    if check_row(&mut v3) == None {
                        out += 1;
                        continue 'a;
                    };
                }
            }
            if x == 2 {
                let mut v4 = row[(1)..].windows(2).map(|a| (a[1] - a[0]));
                if check_row(&mut v4) == None {
                    out += 1;
                    continue 'a;
                }
            }
        } else {
            out += 1;
        }
    }
    out
}
