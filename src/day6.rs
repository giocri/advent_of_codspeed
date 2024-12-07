use std::{
    char,
    cmp::{self, Ordering},
    collections::HashMap,
    fmt::Display,
    io::Cursor,
    path, usize,
};
const COLUMS: usize = 130;
const ROWS: usize = 130;
pub fn part2(input: &str) -> impl Display {
    let mut cusor_pos_x = 0;
    let mut cusor_pos_y = 0;
    let mut cusor_dir = 0;
    let mut map = [0u8; COLUMS * ROWS];
    let mut path = [8u8; COLUMS * ROWS];
    let mut obstacles_v: Vec<Vec<usize>> = vec![Vec::new(); COLUMS];
    let mut obstacles_h: Vec<Vec<usize>> = vec![Vec::new(); ROWS];
    for (pos, char) in input
        .bytes()
        .filter(|&a| a != b'\n' && a != b'\r')
        .enumerate()
    {
        match char {
            b'#' => {
                map[pos] = 255;
                let x = pos % COLUMS;
                let y = pos / COLUMS;
                obstacles_h[y].push(x);
                obstacles_v[x].push(y);
            }
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
        .map(|(i, &dir)| {
            if dir != 8 {
                let x = i % COLUMS;
                let y = i / COLUMS;
                better_loop_check(
                    start_cursor,
                    start_x,
                    start_y,
                    x,
                    y,
                    &obstacles_v,
                    &obstacles_h,
                )
            } else {
                0
            }
        })
        .fold(0, |acc, i| acc + i);
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
fn better_loop_check(
    dir: u8,
    pos_x: usize,
    pos_y: usize,
    pos_obstacle_x: usize,
    pos_obstacle_y: usize,
    obstacles_v: &Vec<Vec<usize>>,
    obstacles_h: &Vec<Vec<usize>>,
) -> u32 {
    let mut cusor_dir = dir;
    let mut cusor_pos_y = pos_y;
    let mut cusor_pos_x = pos_x;
    let mut local_path = [8u8; COLUMS * ROWS];
    'a: loop {
        match cusor_dir {
            0 => {
                if let Some(mut y_obstacle) = searc(&obstacles_v[cusor_pos_x], cusor_pos_y, false) {
                    if pos_obstacle_x == cusor_pos_x && pos_obstacle_y < cusor_pos_y {
                        y_obstacle = cmp::max(y_obstacle, pos_obstacle_y);
                    }
                    let y = y_obstacle + 1;
                    if local_path[y * COLUMS + cusor_pos_x] != 8 {
                        return 1;
                    }
                    local_path[y * COLUMS + cusor_pos_x] = 0;
                    cusor_pos_y = y;
                    cusor_dir = 1;
                    continue 'a;
                } else if pos_obstacle_x == cusor_pos_x && pos_obstacle_y < cusor_pos_y {
                    let y = pos_obstacle_y + 1;
                    if local_path[y * COLUMS + cusor_pos_x] != 8 {
                        return 1;
                    }
                    local_path[y * COLUMS + cusor_pos_x] = 0;
                    cusor_pos_y = y;
                    cusor_dir = 1;
                    continue 'a;
                }
                break 'a;
            }
            1 => {
                if let Some(mut x_obstacle) = searc(&obstacles_h[cusor_pos_y], cusor_pos_x, true) {
                    if pos_obstacle_x == cusor_pos_y && pos_obstacle_x > cusor_pos_x {
                        x_obstacle = cmp::min(x_obstacle, pos_obstacle_x);
                    }
                    let x = x_obstacle - 1;
                    if local_path[cusor_pos_y * COLUMS + x] != 8 {
                        return 1;
                    }
                    local_path[cusor_pos_y * COLUMS + x] = 1;
                    cusor_pos_x = x;
                    cusor_dir = 2;
                    continue 'a;
                } else if pos_obstacle_y == cusor_pos_y && pos_obstacle_x > cusor_pos_x {
                    let x = pos_obstacle_x - 1;
                    if local_path[cusor_pos_y * COLUMS + x] != 8 {
                        return 1;
                    }
                    local_path[cusor_pos_y * COLUMS + x] = 1;
                    cusor_pos_x = x;
                    cusor_dir = 2;
                    continue 'a;
                }
                break 'a;
            }
            2 => {
                if let Some(mut y_obstacle) = searc(&obstacles_v[cusor_pos_x], cusor_pos_y, true) {
                    if pos_obstacle_x == cusor_pos_x && pos_obstacle_y > cusor_pos_y {
                        y_obstacle = cmp::min(y_obstacle, pos_obstacle_y);
                    }
                    let y = y_obstacle - 1;
                    if local_path[y * COLUMS + cusor_pos_x] != 8 {
                        return 1;
                    }
                    local_path[y * COLUMS + cusor_pos_x] = 2;
                    cusor_pos_y = y;
                    cusor_dir = 3;
                    continue 'a;
                } else if pos_obstacle_x == cusor_pos_x && pos_obstacle_y > cusor_pos_y {
                    let y = pos_obstacle_y - 1;
                    if local_path[y * COLUMS + cusor_pos_x] != 8 {
                        return 1;
                    }
                    local_path[y * COLUMS + cusor_pos_x] = 2;
                    cusor_pos_y = y;
                    cusor_dir = 3;
                    continue 'a;
                }
                break 'a;
            }
            3 => {
                if let Some(mut x_obstacle) = searc(&obstacles_h[cusor_pos_y], cusor_pos_x, false) {
                    if pos_obstacle_x == cusor_pos_y && pos_obstacle_x < cusor_pos_x {
                        x_obstacle = cmp::max(x_obstacle, pos_obstacle_x);
                    }
                    let x = x_obstacle + 1;
                    if local_path[cusor_pos_y * COLUMS + x] != 8 {
                        return 1;
                    }
                    local_path[cusor_pos_y * COLUMS + x] = 3;
                    cusor_pos_x = x;
                    cusor_dir = 0;
                    continue 'a;
                } else if pos_obstacle_y == cusor_pos_y && pos_obstacle_x < cusor_pos_x {
                    let x = pos_obstacle_x + 1;
                    if local_path[cusor_pos_y * COLUMS + x] != 8 {
                        return 1;
                    }
                    local_path[cusor_pos_y * COLUMS + x] = 3;
                    cusor_pos_x = x;
                    cusor_dir = 0;
                    continue 'a;
                }
                break 'a;
            }
            _ => {}
        }
    }
    0
}
fn searc(v: &Vec<usize>, pos: usize, greater: bool) -> Option<usize> {
    let mut low = 0;
    let mut high = v.len() - 1;
    let mut best_fit = None;
    while low <= high {
        let mid = (low + high) / 2;

        match v[mid].cmp(&pos) {
            Ordering::Less => {
                low = mid + 1;
                if !greater {
                    best_fit = Some(v[mid]);
                }
            }
            Ordering::Greater => {
                if greater {
                    best_fit = Some(v[mid]);
                }
                if mid == 0 {
                    break;
                }
                high = mid - 1;
            }
            Ordering::Equal => return Some(v[mid]),
        }
    }
    best_fit

    /*if greater {
        v.iter().map(|&a| a).filter(|&a| a > pos).next()
    } else {
        v.iter().rev().map(|&a| a).filter(|&a| a < pos).next()
    }*/
}
