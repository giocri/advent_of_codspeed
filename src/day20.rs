const CUTOFF: i32 = 100;
const COLUMS: usize = 141;
const ROWS: usize = 141;
const ROWSIZE: usize = 142;
const DIRECTIONS: [i32; 4] = [1, -(ROWSIZE as i32), -1, (ROWSIZE as i32)];
pub fn part1(input: &str) -> u32 {
    let map = input.as_bytes();
    let mut cost_map = [i32::MAX; ROWS * ROWSIZE];
    let end = map.iter().position(|&a| a == b'E').unwrap();
    //let start = map.iter().position(|&a| a == b'S').unwrap();
    cost_map[end] = 0;
    let mut previous = end;
    let mut cost = 0;
    'a: loop {
        for d in DIRECTIONS {
            let check = (d + previous as i32) as usize;
            /*println!(
                "{} {} {} {}",
                map[check] as char,
                cost_map[check],
                check / ROWSIZE,
                check % ROWSIZE
            );*/
            if map[check] != b'#' && cost_map[check] >= cost {
                cost_map[check] = cost + 1;
                previous = check;
                cost += 1;
                continue 'a;
            }
        }
        break 'a;
    }
    /*for y in 1..(ROWS - 1) {
        for x in 1..(COLUMS - 1) {
            println!("{} {} {}", x, y, cost_map[(y * ROWSIZE) + x]);
        }
    }*/
    let mut out = 0;
    for y in 1..(ROWS - 1) {
        for x in 1..(COLUMS - 1) {
            if x < COLUMS - 2 {
                let map_1 = map[(y * ROWSIZE) + x];
                let map_2 = map[(y * ROWSIZE) + x + 2];
                let map_3 = map[(y * ROWSIZE) + x + 1];
                if map_1 != b'#'
                    && map_2 != b'#'
                    && map_3 == b'#'
                    && (cost_map[(y * ROWSIZE) + x] - cost_map[(y * ROWSIZE) + x + 2]).abs() - 2
                        >= CUTOFF
                {
                    //println!("jump x! x: {} y: {}", x, y);
                    out += 1;
                }
            }
            if y < ROWS - 2 {
                let map_1 = map[(y * ROWSIZE) + x];
                let map_2 = map[((y + 2) * ROWSIZE) + x];
                let map_3 = map[((y + 1) * ROWSIZE) + x];
                if map_1 != b'#'
                    && map_2 != b'#'
                    && map_3 == b'#'
                    && (cost_map[(y * ROWSIZE) + x] - cost_map[((y + 2) * ROWSIZE) + x]).abs() - 2
                        >= CUTOFF
                {
                    //println!("jump y! x: {} y: {}", x, y);
                    out += 1;
                }
            }
            let map_1 = map[(y * ROWSIZE) + x];
            let map_2 = map[((y + 1) * ROWSIZE) + x + 1];
            if map_1 != b'#'
                && map_2 != b'#'
                && (cost_map[(y * ROWSIZE) + x] - cost_map[((y + 1) * ROWSIZE) + x + 1]).abs() - 2
                    >= CUTOFF
            {
                //println!("jump diag+! x: {} y: {}", x, y);
                out += 1;
            }
            let map_1 = map[(y * ROWSIZE) + x + 1];
            let map_2 = map[((y + 1) * ROWSIZE) + x];
            if map_1 != b'#'
                && map_2 != b'#'
                && (cost_map[(y * ROWSIZE) + x + 1] - cost_map[((y + 1) * ROWSIZE) + x]).abs() - 2
                    >= CUTOFF
            {
                //println!("jump diag-! x: {} y: {}", x, y);
                out += 1;
            }
        }
    }
    out
}
pub fn part2(input: &str) -> u32 {
    let map = input.as_bytes();
    let mut cost_map = [i32::MAX; ROWS * ROWSIZE];
    let end = map.iter().position(|&a| a == b'E').unwrap();
    //let start = map.iter().position(|&a| a == b'S').unwrap();
    cost_map[end] = 0;
    let mut previous = end;
    let mut cost = 0;
    'a: loop {
        for d in DIRECTIONS {
            let check = (d + previous as i32) as usize;
            if map[check] != b'#' && cost_map[check] >= cost {
                cost_map[check] = cost + 1;
                previous = check;
                cost += 1;
                continue 'a;
            }
        }
        break 'a;
    }
    /*for y in 0..ROWS {
        for x in 0..COLUMS {
            let i1 = (ROWSIZE * y) + x;
            if map[i1] != b'#' && cost_map[i1] == i32::MAX {
                println!("panic {} {}", x, y);
            }
        }
    }*/
    let mut out = 0;
    for y in 1..(ROWS - 1) {
        for x in 1..(COLUMS - 1) {
            for dy in 0..=20 {
                if y + dy > (ROWS - 2) {
                    continue;
                }
                let i1 = (ROWSIZE * y) + x;
                if map[i1] != b'#' {
                    for dx in 0..=(20 - dy) {
                        if x + dx < (COLUMS - 1) {
                            let i2 = (ROWSIZE * (y + dy)) + x + dx;
                            if map[i2] != b'#'
                                && (cost_map[i1] - cost_map[i2]).abs() - (dx + dy) as i32 >= CUTOFF
                            {
                                out += 1;
                            }
                        }
                    }
                }
                if dy > 0 {
                    for dx in 1..=(20 - dy) {
                        if x + dx < (COLUMS - 1) {
                            let i4 = (ROWSIZE * (y + dy)) + x;
                            let i3 = (ROWSIZE * y) + x + dx;
                            if map[i3] != b'#'
                                && map[i4] != b'#'
                                && (cost_map[i3] - cost_map[i4]).abs() - (dx + dy) as i32 >= CUTOFF
                            {
                                out += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    //println!("max {}", hash_map.into_values().max().unwrap());
    out
}
