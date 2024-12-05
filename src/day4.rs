use std::{fmt::Display, iter, usize};
use wide::{i32x4, i32x8, u8x16, CmpGt, CmpLt};
const ROWS: i32 = 140;
const COLUMNS: i32 = 140;
const ROWSIZE: i32 = 141 + if cfg!(target_os = "windows") { 1 } else { 0 };

pub fn part2(input: &str) -> impl Display {
    let mut out = 0;
    let bytes = input.as_bytes();
    for i in 1..ROWS - 1 {
        let row_start = i * ROWSIZE;
        let xes = (1..(COLUMNS as usize - 1)).filter(|c| bytes[row_start as usize + c] == b'A');
        let row_matches = xes
            .filter(|&x| check_x(bytes, row_start + x as i32))
            .count();
        out += row_matches;
    }
    out
}

pub fn part1(input: &str) -> impl Display {
    let mut out = 0;
    let bytes = input.as_bytes();
    let upperguard = 2;
    let lowerguard = ROWS - 3;
    for i in 0..ROWS {
        let mut row_matches = 0;
        let row_start = i * ROWSIZE;
        let xes = (0..(COLUMNS as usize)).filter(|c| bytes[row_start as usize + c] == b'X');
        if i > upperguard {
            row_matches += xes
                .clone()
                .filter(|&x| check_match_up(bytes, row_start + x as i32))
                .count();
            row_matches += xes
                .clone()
                .filter(|&x| x > 2 && check_match_diag_up_left(bytes, row_start + x as i32))
                .count();
            row_matches += xes
                .clone()
                .filter(|&x| {
                    x < (COLUMNS as usize - 3)
                        && check_match_diag_up_right(bytes, row_start + x as i32)
                })
                .count();
        }
        if i < lowerguard {
            row_matches += xes
                .clone()
                .filter(|&x| check_match_down(bytes, row_start + x as i32))
                .count();
            row_matches += xes
                .clone()
                .filter(|&x| {
                    x as usize > 2 && check_match_diag_down_left(bytes, row_start + x as i32)
                })
                .count();
            row_matches += xes
                .clone()
                .filter(|&x| {
                    x < (COLUMNS as usize - 3)
                        && check_match_diag_down_right(bytes, row_start + x as i32)
                })
                .count();
        }
        row_matches += xes
            .clone()
            .filter(|&x| {
                x > 2
                    && bytes[(row_start as usize + x - 3)..(row_start as usize + x)].as_ref()
                        == "SAM".as_bytes()
            })
            .count();
        row_matches += xes
            .clone()
            .filter(|&x| {
                x < (COLUMNS as usize - 3)
                    && bytes[(row_start as usize + x + 1)..(row_start as usize + x + 4)].as_ref()
                        == "MAS".as_bytes()
            })
            .count();
        out += row_matches;
    }
    out
}
const fn offset(rows_offset: i32, colums_offset: i32) -> i32 {
    return rows_offset * (ROWSIZE as i32) + colums_offset;
}
fn check_match_up(data: &[u8], i: i32) -> bool {
    data[(i + offset(-1, 0)) as usize] == b'M'
        && data[(i + offset(-2, 0)) as usize] == b'A'
        && data[(i + offset(-3, 0)) as usize] == b'S'
}
fn check_match_down(data: &[u8], i: i32) -> bool {
    data[(i + offset(1, 0)) as usize] == b'M'
        && data[(i + offset(2, 0)) as usize] == b'A'
        && data[(i + offset(3, 0)) as usize] == b'S'
}
fn check_match_diag_down_left(data: &[u8], i: i32) -> bool {
    data[(i + offset(1, -1)) as usize] == b'M'
        && data[(i + offset(2, -2)) as usize] == b'A'
        && data[(i + offset(3, -3)) as usize] == b'S'
}
fn check_match_diag_down_right(data: &[u8], i: i32) -> bool {
    data[(i + offset(1, 1)) as usize] == b'M'
        && data[(i + offset(2, 2)) as usize] == b'A'
        && data[(i + offset(3, 3)) as usize] == b'S'
}
fn check_match_diag_up_left(data: &[u8], i: i32) -> bool {
    data[(i + offset(-1, -1)) as usize] == b'M'
        && data[(i + offset(-2, -2)) as usize] == b'A'
        && data[(i + offset(-3, -3)) as usize] == b'S'
}
fn check_match_diag_up_right(data: &[u8], i: i32) -> bool {
    data[(i + offset(-1, 1)) as usize] == b'M'
        && data[(i + offset(-2, 2)) as usize] == b'A'
        && data[(i + offset(-3, 3)) as usize] == b'S'
}
fn check_x(data: &[u8], i: i32) -> bool {
    ((data[(i + offset(-1, -1)) as usize] == b'M' && data[(i + offset(1, 1)) as usize] == b'S')
        || (data[(i + offset(-1, -1)) as usize] == b'S'
            && data[(i + offset(1, 1)) as usize] == b'M'))
        && ((data[(i + offset(-1, 1)) as usize] == b'M'
            && data[(i + offset(1, -1)) as usize] == b'S')
            || (data[(i + offset(-1, 1)) as usize] == b'S'
                && data[(i + offset(1, -1)) as usize] == b'M'))
}
fn check_simd(data: &[u8], start: i32, len: i32, offsets: [i32; 3]) -> usize {
    let iter = (start..(start + len)).step_by(16);
    let mask = u8x16::ONE;
    let x = u8x16::from(b'X');
    let m = u8x16::from(b'M');
    let a = u8x16::from(b'A');
    let s = u8x16::from(b'S');
    let check = [m, a, s];
    let mut out = 0;
    for p in iter.clone() {
        let start = p as usize;
        let mut valid = x.cmp_eq(u8x16::from(&data[start..(start + 8)]));
        for i in 0..3 {
            let start = (p + offsets[i]) as usize;
            let tmp = check[i].cmp_eq(u8x16::from(&data[start..(start + 8)]));
            valid &= tmp;
        }
        out += (valid & mask).to_array().iter().fold(0, |acc, x| acc + x) as usize;
    }
    if len % 16 == 0 {
        return out;
    }
    out += (((len / 16) * 16)..len)
        .filter(|&p| {
            let p = p + start;
            data[p as usize] == b'X'
                && data[(p + offsets[0]) as usize] == b'M'
                && data[(p + offsets[1]) as usize] == b'A'
                && data[(p + offsets[2]) as usize] == b'S'
        })
        .count();
    out
}
pub fn part1_1(input: &str) -> impl Display {
    let mut out = 0;
    let data = input.as_bytes();
    let upperguard = 2;
    let lowerguard = ROWS - 3;
    for i in 0..ROWS {
        let mut row_matches = 0;
        let row_start = i * ROWSIZE;
        if i > upperguard {
            row_matches += check_simd(
                data,
                row_start,
                COLUMNS,
                [offset(-1, 0), offset(-2, 0), offset(-3, 0)],
            );
            row_matches += check_simd(
                data,
                row_start + 3,
                COLUMNS - 3,
                [offset(-1, -1), offset(-2, -2), offset(-3, -3)],
            );
            row_matches += check_simd(
                data,
                row_start,
                COLUMNS - 3,
                [offset(-1, 1), offset(-2, 2), offset(-3, 3)],
            );
        }
        if i < lowerguard {
            row_matches += check_simd(
                data,
                row_start,
                COLUMNS,
                [offset(1, 0), offset(2, 0), offset(3, 0)],
            );
            row_matches += check_simd(
                data,
                row_start + 3,
                COLUMNS - 3,
                [offset(1, -1), offset(2, -2), offset(3, -3)],
            );
            row_matches += check_simd(
                data,
                row_start,
                COLUMNS - 3,
                [offset(1, 1), offset(2, 2), offset(3, 3)],
            );
        }
        row_matches += check_simd(
            data,
            row_start + 3,
            COLUMNS - 3,
            [offset(0, -1), offset(0, -2), offset(0, -3)],
        );
        row_matches += check_simd(
            data,
            row_start,
            COLUMNS - 3,
            [offset(0, 1), offset(0, 2), offset(0, 3)],
        );
        out += row_matches;
    }
    out
}
