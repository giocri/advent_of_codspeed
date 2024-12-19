use std::{collections::BinaryHeap, fmt::Display};
//const COLUMS: usize = 15;
//const ROWS: usize = 15;
const ROWSIZE: usize = 142;
const DIRECTIONS: [i32; 4] = [1, -(ROWSIZE as i32), -1, (ROWSIZE as i32)];
pub fn part1(input: &str) -> impl Display {
    let map = input.as_bytes();
    let mut cost_map = [i32::MIN; ROWSIZE * ROWSIZE];
    //let end = map.iter().position(|&a| a == b'E').unwrap();
    let start = map.iter().position(|&a| a == b'S').unwrap();
    let mut to_check = BinaryHeap::new();
    let mut final_score =
        find_paths_from_point(start as i32, 0, map, &mut to_check, 0, &mut cost_map);
    while final_score == None {
        let (best_score, position, direction) = to_check.pop().unwrap();
        final_score = find_paths_from_point(
            position,
            direction,
            map,
            &mut to_check,
            best_score,
            &mut cost_map,
        );
    }
    final_score.unwrap() * -1
}
pub fn part2(input: &str) -> impl Display {
    let map = input.as_bytes();
    let mut cost_map = [i32::MIN; ROWSIZE * ROWSIZE];
    let mut pevious_tiles: Vec<(usize, i32, usize, i32)> = Vec::new();
    pevious_tiles.push((0, 0, 0, 0));
    //let end = map.iter().position(|&a| a == b'E').unwrap();
    let start = map.iter().position(|&a| a == b'S').unwrap();
    let mut to_check: BinaryHeap<(i32, i32, usize, usize)> = BinaryHeap::new();
    let mut final_score = find_paths_from_point_track(
        start as i32,
        0,
        map,
        &mut to_check,
        0,
        &mut cost_map,
        &mut pevious_tiles,
        0,
    );
    while final_score == None {
        let (best_score, position, direction, tiles) = to_check.pop().unwrap();
        final_score = find_paths_from_point_track(
            position,
            direction,
            map,
            &mut to_check,
            best_score,
            &mut cost_map,
            &mut pevious_tiles,
            tiles,
        );
    }
    let mut optimal_paths: Vec<usize> = Vec::new();
    optimal_paths.push(pevious_tiles.len() - 1);
    let optimal_score = final_score.unwrap();
    //let mut found_score: Option<i32> = None;
    loop {
        let mut found_score = None;
        while found_score == None {
            let (best_score, position, direction, tiles) = to_check.pop().unwrap();
            found_score = find_paths_from_point_track(
                position,
                direction,
                map,
                &mut to_check,
                best_score,
                &mut cost_map,
                &mut pevious_tiles,
                tiles,
            );
        }
        if found_score.unwrap() < optimal_score - 1000 {
            break;
        }
        if found_score.unwrap() == optimal_score {
            optimal_paths.push(pevious_tiles.len() - 1);
        }
    }
    let mut in_path = [0u8; ROWSIZE * ROWSIZE];
    for p in optimal_paths {
        let mut head = p;
        while head != 0 {
            let (previous, start, dir, distance) = pevious_tiles[head];
            for i in 0..=distance {
                in_path[(start + DIRECTIONS[dir] * i) as usize] = 1
            }
            head = previous;
        }
    }
    in_path.iter().fold(0u32, |acc, x| acc + *x as u32) + 1
}
fn find_paths_from_point(
    start: i32,
    direction: usize,
    map: &[u8],
    deposit: &mut BinaryHeap<(i32, i32, usize)>,
    stating_score: i32,
    cost_map: &mut [i32],
) -> Option<i32> {
    let back = (direction + 2) % 4;
    'a: for d in 0..4 {
        if d == back {
            continue;
        }
        let turn_score = if d == direction { 0 } else { 1000 };

        match map[(start + DIRECTIONS[d]) as usize] {
            b'#' => {
                continue 'a;
            }
            b'E' => {
                return Some(stating_score - 1 - turn_score);
            }
            _ => {}
        }
        let mut distance = 1;
        loop {
            let cursor = start as i32 + DIRECTIONS[d] * distance;
            //println!("{} {}", cursor, direction);
            let left = map[(cursor + DIRECTIONS[(d + 1) % 4]) as usize];
            let right = map[(cursor + DIRECTIONS[(d + 3) % 4]) as usize];
            let forward = map[(cursor + DIRECTIONS[d]) as usize];
            //println!("{} {} {}", left as char, right as char, forward as char);
            //let mut s = String::new();
            //stdin().read_line(&mut s).unwrap();
            match (left, right, forward) {
                (b'#', b'#', b'#') => {
                    continue 'a;
                }
                (_, _, b'E') => {
                    return Some(stating_score - distance - 1 - turn_score);
                }
                (b'.' | b'E', _, _) | (_, b'.' | b'E', _) => {
                    let score = stating_score - distance - turn_score;
                    let old_cost = cost_map[cursor as usize];
                    if old_cost <= score + 1000 {
                        deposit.push((score, cursor, d));
                        if old_cost < score - 1000 {
                            cost_map[cursor as usize] = score;
                        }
                    }
                    continue 'a;
                }
                _ => {
                    distance += 1;
                }
            }
        }
    }
    None
}
fn find_paths_from_point_track(
    start: i32,
    direction: usize,
    map: &[u8],
    deposit: &mut BinaryHeap<(i32, i32, usize, usize)>,
    stating_score: i32,
    cost_map: &mut [i32],
    previous_tiles: &mut Vec<(usize, i32, usize, i32)>,
    previous_node: usize,
) -> Option<i32> {
    let back = (direction + 2) % 4;
    'a: for d in 0..4 {
        if d == back {
            continue;
        }
        let turn_score = if d == direction { 0 } else { 1000 };

        match map[(start + DIRECTIONS[d]) as usize] {
            b'#' => {
                continue 'a;
            }
            b'E' => {
                previous_tiles.push((previous_node, start, d, 1));
                return Some(stating_score - 1 - turn_score);
            }
            _ => {}
        }
        let mut distance = 1;
        loop {
            let cursor = start as i32 + DIRECTIONS[d] * distance;
            //println!("{} {}", cursor, direction);
            let left = map[(cursor + DIRECTIONS[(d + 1) % 4]) as usize];
            let right = map[(cursor + DIRECTIONS[(d + 3) % 4]) as usize];
            let forward = map[(cursor + DIRECTIONS[d]) as usize];
            //println!("{} {} {}", left as char, right as char, forward as char);
            //let mut s = String::new();
            //stdin().read_line(&mut s).unwrap();
            match (left, right, forward) {
                (b'#', b'#', b'#') => {
                    continue 'a;
                }
                (_, _, b'E') => {
                    previous_tiles.push((previous_node, start, d, distance));
                    return Some(stating_score - distance - 1 - turn_score);
                }
                (b'.' | b'E', _, _) | (_, b'.' | b'E', _) => {
                    let score = stating_score - distance - turn_score;
                    let old_cost = cost_map[cursor as usize];
                    if old_cost <= score + 2000 {
                        deposit.push((score, cursor, d, previous_tiles.len()));
                        previous_tiles.push((previous_node, start, d, distance));
                        if old_cost < score - 2000 {
                            cost_map[cursor as usize] = score;
                        }
                    }
                    continue 'a;
                }
                _ => {
                    distance += 1;
                }
            }
        }
    }
    None
}
