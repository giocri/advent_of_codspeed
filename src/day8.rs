use std::{fmt::Display, iter};

use num::integer::gcd;
const COLUMS: i32 = 50;
const ROWS: i32 = 50;
const ROWSIZE: i32 = 51;
pub fn part1(input: &str) -> impl Display {
    let mut frequencies: Vec<Vec<(i32, i32)>> = vec![Vec::new(); 62];
    let mapping_iter = input.as_bytes().iter().enumerate().filter_map(|(pos, &a)| {
        let x = pos as i32 % ROWSIZE;
        let y = pos as i32 / ROWSIZE;
        match a {
            48..=57 => Some((a - 48, x, y)),
            65..=90 => Some((a - 55, x, y)),
            97..=122 => Some((a - 61, x, y)),
            _ => None,
        }
    });
    for a in mapping_iter {
        frequencies[a.0 as usize].push((a.1, a.2));
    }
    let mut out = [0; (COLUMS * ROWS) as usize];
    for f in frequencies {
        for (i, (x1, y1)) in f.iter().enumerate() {
            for (x2, y2) in f[i + 1..].iter() {
                let diff_x = x2 - x1;
                let diff_y = y2 - y1;
                if diff_x.abs() > COLUMS / 2 || diff_y.abs() > ROWS / 2 {
                    continue;
                } else {
                    let antinode1_x = x1 - diff_x;
                    let antinode1_y = y1 - diff_y;
                    let antinode2_x = x2 + diff_x;
                    let antinode2_y = y2 + diff_y;
                    if (antinode1_x >= 0 && antinode1_x < COLUMS)
                        && (antinode1_y >= 0 && antinode1_y < ROWS)
                    {
                        out[(antinode1_y * COLUMS + antinode1_x) as usize] = 1;
                    }
                    if (antinode2_x >= 0 && antinode2_x < COLUMS)
                        && (antinode2_y >= 0 && antinode2_y < ROWS)
                    {
                        out[(antinode2_y * COLUMS + antinode2_x) as usize] = 1;
                    }
                }
            }
        }
    }
    out.iter().fold(0, |acc, a| acc + a)
}
pub fn part2(input: &str) -> impl Display {
    let mut frequencies: Vec<Vec<(i32, i32)>> = vec![Vec::new(); 62];
    let mapping_iter = input.as_bytes().iter().enumerate().filter_map(|(pos, &a)| {
        let x = pos as i32 % ROWSIZE;
        let y = pos as i32 / ROWSIZE;
        match a {
            48..=57 => Some((a - 48, x, y)),
            65..=90 => Some((a - 55, x, y)),
            97..=122 => Some((a - 61, x, y)),
            _ => None,
        }
    });
    for a in mapping_iter {
        frequencies[a.0 as usize].push((a.1, a.2));
    }
    let mut out = [0; (COLUMS * ROWS) as usize];
    for f in frequencies {
        for (i, (x1, y1)) in f.iter().enumerate() {
            for (x2, y2) in f[i + 1..].iter() {
                let mut diff_x = x2 - x1;
                let mut diff_y = y2 - y1;
                let gcd = gcd(diff_x, diff_y);
                diff_x /= gcd;
                diff_y /= gcd;
                let mut antinode1_x = *x1;
                let mut antinode1_y = *y1;
                let mut antinode2_x = *x1;
                let mut antinode2_y = *y1;
                while (antinode1_x >= 0 && antinode1_x < COLUMS)
                    && (antinode1_y >= 0 && antinode1_y < ROWS)
                {
                    out[(antinode1_y * COLUMS + antinode1_x) as usize] = 1;
                    antinode1_x -= diff_x;
                    antinode1_y -= diff_y;
                }
                while (antinode2_x >= 0 && antinode2_x < COLUMS)
                    && (antinode2_y >= 0 && antinode2_y < ROWS)
                {
                    out[(antinode2_y * COLUMS + antinode2_x) as usize] = 1;
                    antinode2_x += diff_x;
                    antinode2_y += diff_y;
                }
            }
        }
    }
    out.iter().fold(0, |acc, a| acc + a)
}
