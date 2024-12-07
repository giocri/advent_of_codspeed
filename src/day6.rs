use std::{char, cmp, collections::HashMap, fmt::Display, io::Cursor, path};
const COLUMS: usize = 130;
const ROWS: usize = 130;
pub fn part2(input: &str) -> impl Display {
    let mut cusor_pos_x = 0;
    let mut cusor_pos_y = 0;
    let mut cusor_dir = 0;
    let mut map = [0u8; COLUMS * ROWS];
    let mut path = [8u8; COLUMS * ROWS];
    let mut obstacles = [0u8; COLUMS * ROWS];
    for (pos, char) in input
        .bytes()
        .filter(|&a| a != b'\n' && a != b'\r')
        .enumerate()
    {
        match char {
            b'#' => map[pos] = 255,
            b'^' => {
                cusor_pos_x = pos % COLUMS;
                cusor_pos_y = pos / COLUMS;
            }
            b'>' => {
                cusor_pos_x = pos % COLUMS;
                cusor_pos_y = pos / COLUMS;
                cusor_dir = 1;
            }
            b'v' => {
                cusor_pos_x = pos % COLUMS;
                cusor_pos_y = pos / COLUMS;
                cusor_dir = 2;
            }
            b'<' => {
                cusor_pos_x = pos % COLUMS;
                cusor_pos_y = pos / COLUMS;
                cusor_dir = 3;
            }
            _ => {}
        }
    }
    let start_x = cusor_pos_x;
    let start_y = cusor_pos_y;
    let start_cursor = cusor_dir;
    let mut out = 0;
    'a: loop {
        match cusor_dir {
            0 => {
                for y in (0..cusor_pos_y).rev() {
                    if map[y * COLUMS + cusor_pos_x] == 0 {
                        path[y * COLUMS + cusor_pos_x] = 0;
                    } else {
                        cusor_pos_y = y + 1;
                        cusor_dir = 1;
                        continue 'a;
                    }
                }
                break 'a;
            }
            1 => {
                for x in cusor_pos_x + 1..COLUMS {
                    if map[cusor_pos_y * COLUMS + x] == 0 {
                        path[cusor_pos_y * COLUMS + x] = 1;
                    } else {
                        cusor_pos_x = x - 1;
                        cusor_dir = 2;
                        continue 'a;
                    }
                }
                break 'a;
            }
            2 => {
                for y in cusor_pos_y + 1..ROWS {
                    if map[y * COLUMS + cusor_pos_x] == 0 {
                        path[y * COLUMS + cusor_pos_x] = 2;
                    } else {
                        cusor_pos_y = y - 1;
                        cusor_dir = 3;
                        continue 'a;
                    }
                }
                break 'a;
            }
            3 => {
                for x in (0..cusor_pos_x).rev() {
                    if map[cusor_pos_y * COLUMS + x] == 0 {
                        path[cusor_pos_y * COLUMS + x] = 3;
                    } else {
                        cusor_pos_x = x + 1;
                        cusor_dir = 0;
                        continue 'a;
                    }
                }
                break 'a;
            }
            _ => {}
        }
    }
    let out = path
        .iter()
        .enumerate()
        .filter(|(i, &x)| {
            if x != 8 {
                map[*i] = 255;
                let obstacle = loop_check(&map, start_cursor, start_x, start_y) > 0;
                map[*i] = 0;
                obstacle
            } else {
                false
            }
        })
        .count();
    out
}
pub fn part1(input: &str) -> impl Display {
    let mut cusor_pos_x = 0;
    let mut cusor_pos_y = 0;
    let mut cusor_dir = 0;
    let mut map = [0u8; COLUMS * ROWS];
    let mut path = [0u8; COLUMS * ROWS];
    for (pos, char) in input
        .bytes()
        .filter(|&a| a != b'\n' && a != b'\r')
        .enumerate()
    {
        match char {
            b'#' => map[pos] = 255,
            b'^' => {
                cusor_pos_x = pos % COLUMS;
                cusor_pos_y = pos / COLUMS;
            }
            b'>' => {
                cusor_pos_x = pos % COLUMS;
                cusor_pos_y = pos / COLUMS;
                cusor_dir = 1;
            }
            b'v' => {
                cusor_pos_x = pos % COLUMS;
                cusor_pos_y = pos / COLUMS;
                cusor_dir = 2;
            }
            b'<' => {
                cusor_pos_x = pos % COLUMS;
                cusor_pos_y = pos / COLUMS;
                cusor_dir = 3;
            }
            _ => {}
        }
    }
    'a: loop {
        match cusor_dir {
            0 => {
                for y in (0..cusor_pos_y).rev() {
                    if map[y * COLUMS + cusor_pos_x] == 0 {
                        path[y * COLUMS + cusor_pos_x] = 1;
                    } else {
                        cusor_pos_y = y + 1;
                        cusor_dir = 1;
                        continue 'a;
                    }
                }
                break 'a;
            }
            1 => {
                for x in cusor_pos_x + 1..COLUMS {
                    if map[cusor_pos_y * COLUMS + x] == 0 {
                        path[cusor_pos_y * COLUMS + x] = 1;
                    } else {
                        cusor_pos_x = x - 1;
                        cusor_dir = 2;
                        continue 'a;
                    }
                }
                break 'a;
            }
            2 => {
                for y in cusor_pos_y + 1..ROWS {
                    if map[y * COLUMS + cusor_pos_x] == 0 {
                        path[y * COLUMS + cusor_pos_x] = 1;
                    } else {
                        cusor_pos_y = y - 1;
                        cusor_dir = 3;
                        continue 'a;
                    }
                }
                break 'a;
            }
            3 => {
                for x in (0..cusor_pos_x).rev() {
                    if map[cusor_pos_y * COLUMS + x] == 0 {
                        path[cusor_pos_y * COLUMS + x] = 1;
                    } else {
                        cusor_pos_x = x + 1;
                        cusor_dir = 0;
                        continue 'a;
                    }
                }
                break 'a;
            }
            _ => {}
        }
    }
    let out = path.iter().map(|&a| a as u32).fold(0, |acc, i| acc + i);
    out
}

fn loop_check(map: &[u8], dir: u8, pos_x: usize, pos_y: usize) -> u8 {
    let mut cusor_dir = dir;
    let mut cusor_pos_y = pos_y;
    let mut cusor_pos_x = pos_x;
    let mut local_path = [8u8; COLUMS * ROWS];
    'a: loop {
        match cusor_dir {
            0 => {
                for y in (0..cusor_pos_y).rev() {
                    if map[y * COLUMS + cusor_pos_x] == 0 {
                        if local_path[y * COLUMS + cusor_pos_x] == 0 {
                            return 1;
                        }
                        local_path[y * COLUMS + cusor_pos_x] = 0;
                    } else {
                        cusor_pos_y = y + 1;
                        cusor_dir = 1;
                        continue 'a;
                    }
                }
                break 'a;
            }
            1 => {
                for x in cusor_pos_x + 1..COLUMS {
                    if map[cusor_pos_y * COLUMS + x] == 0 {
                        if local_path[cusor_pos_y * COLUMS + x] == 1 {
                            return 1;
                        }
                        local_path[cusor_pos_y * COLUMS + x] = 1;
                    } else {
                        cusor_pos_x = x - 1;
                        cusor_dir = 2;
                        continue 'a;
                    }
                }
                break 'a;
            }
            2 => {
                for y in cusor_pos_y + 1..ROWS {
                    if map[y * COLUMS + cusor_pos_x] == 0 {
                        if local_path[y * COLUMS + cusor_pos_x] == 2 {
                            return 1;
                        }
                        local_path[y * COLUMS + cusor_pos_x] = 2;
                    } else {
                        cusor_pos_y = y - 1;
                        cusor_dir = 3;
                        continue 'a;
                    }
                }
                break 'a;
            }
            3 => {
                for x in (0..cusor_pos_x).rev() {
                    if map[cusor_pos_y * COLUMS + x] == 0 {
                        if local_path[cusor_pos_y * COLUMS + x] == 3 {
                            return 1;
                        }
                        local_path[cusor_pos_y * COLUMS + x] = 3;
                    } else {
                        cusor_pos_x = x + 1;
                        cusor_dir = 0;
                        continue 'a;
                    }
                }
                break 'a;
            }
            _ => {}
        }
    }
    0
}
