use std::{fmt::Display, iter, usize};
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
