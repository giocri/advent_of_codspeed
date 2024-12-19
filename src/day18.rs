use std::collections::BinaryHeap;

const COLUMS: usize = 73;
const ROWS: usize = 73;
const CUTOFF: usize = 1024;
const DIRECTIONS: [i32; 4] = [1, -(COLUMS as i32), -1, (COLUMS as i32)];
pub fn part1(input: &str) -> u32 {
    let mut map = [0u8; ROWS * COLUMS];
    let mut lines = input.lines();
    for _ in 0..CUTOFF {
        let mut line = lines.next().unwrap().split(',');
        let x = u32::from_str_radix(line.next().unwrap(), 10).unwrap() as usize + 1;
        let y = u32::from_str_radix(line.next().unwrap(), 10).unwrap() as usize + 1;
        map[x + y * COLUMS] = 1;
    }
    for i in 0..COLUMS {
        map[i] = 1;
        map[i + COLUMS * (ROWS - 1)] = 1;
        map[COLUMS * i] = 1;
        map[COLUMS * i + COLUMS - 1] = 1;
    }
    let mut cost_map = [i32::MIN; ROWS * COLUMS];
    //let end = map.iter().position(|&a| a == 0).unwrap();
    let start = COLUMS + 1;
    //println!("start {}", start);
    let mut to_check = BinaryHeap::new();
    let mut final_score =
        find_paths_from_point(start as i32, 0, &map, &mut to_check, 0, &mut cost_map);
    while final_score == None {
        let (best_score, position, direction) = to_check.pop().unwrap();
        final_score = find_paths_from_point(
            position,
            direction,
            &map,
            &mut to_check,
            best_score,
            &mut cost_map,
        );
    }
    (final_score.unwrap() * -1) as u32
}
pub fn part2(input: &str) -> String {
    let mut map = [0u8; ROWS * COLUMS];
    let mut lines = input.lines();
    for _ in 0..CUTOFF {
        let mut line = lines.next().unwrap().split(',');
        let x = u32::from_str_radix(line.next().unwrap(), 10).unwrap() as usize + 1;
        let y = u32::from_str_radix(line.next().unwrap(), 10).unwrap() as usize + 1;
        map[x + y * COLUMS] = 1;
    }
    for i in 0..COLUMS {
        map[i] = 1;
        map[i + COLUMS * (ROWS - 1)] = 1;
        map[COLUMS * i] = 1;
        map[COLUMS * i + COLUMS - 1] = 1;
    }
    let mut cost_map = [i32::MIN; ROWS * COLUMS];
    //let end = map.iter().position(|&a| a == 0).unwrap();
    let start = COLUMS + 1;
    //println!("start {}", start);
    let mut to_check = BinaryHeap::new();
    let mut pevious_tiles: Vec<(usize, i32, usize, i32)> = Vec::new();
    pevious_tiles.push((0, 0, 0, 0));
    let mut final_score = find_paths_from_point_track(
        start as i32,
        0,
        &map,
        &mut to_check,
        0,
        &mut cost_map,
        &mut pevious_tiles,
        0,
    );
    while final_score == None {
        let (best_score, position, direction, pevious_node) = to_check.pop().unwrap();
        final_score = find_paths_from_point_track(
            position,
            direction,
            &map,
            &mut to_check,
            best_score,
            &mut cost_map,
            &mut pevious_tiles,
            pevious_node,
        );
    }
    let mut final_x;
    let mut final_y;
    'b: loop {
        let mut cost_map = [i32::MIN; ROWS * COLUMS];
        let mut pathmap = [0u8; ROWS * COLUMS];
        let mut head = pevious_tiles.len() - 1;
        while head != 0 {
            let (previous, start, dir, distance) = pevious_tiles[head];
            for i in 0..=distance {
                pathmap[(start + DIRECTIONS[dir] * i) as usize] = 1
            }
            head = previous;
        }
        pevious_tiles.clear();
        to_check.clear();
        pevious_tiles.push((0, 0, 0, 0));
        'a: loop {
            if let Some(l) = lines.next() {
                let mut line = l.split(',');
                let x = u32::from_str_radix(line.next().unwrap(), 10).unwrap() as usize + 1;
                let y = u32::from_str_radix(line.next().unwrap(), 10).unwrap() as usize + 1;
                map[x + y * COLUMS] = 1;
                if pathmap[x + y * COLUMS] == 1 {
                    final_x = x - 1;
                    final_y = y - 1;
                    break 'a;
                }
            }
        }
        let mut final_score = find_paths_from_point_track(
            start as i32,
            0,
            &map,
            &mut to_check,
            0,
            &mut cost_map,
            &mut pevious_tiles,
            0,
        );
        while final_score == None {
            let Some((best_score, position, direction, pevious_node)) = to_check.pop() else {
                break 'b;
            };
            final_score = find_paths_from_point_track(
                position,
                direction,
                &map,
                &mut to_check,
                best_score,
                &mut cost_map,
                &mut pevious_tiles,
                pevious_node,
            );
        }
    }
    (final_x.to_string() + ",") + final_y.to_string().as_str()
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
        match map[(start + DIRECTIONS[d]) as usize] {
            1 => {
                continue 'a;
            }
            _ => {}
        }
        let mut distance = 1;
        loop {
            let cursor = start as i32 + DIRECTIONS[d] * distance;
            if cursor as usize == ((ROWS - 2) * COLUMS) + COLUMS - 2 {
                return Some(stating_score - distance);
            }
            //println!("{} {}", cursor, d);
            let left = map[(cursor + DIRECTIONS[(d + 1) % 4]) as usize];
            let right = map[(cursor + DIRECTIONS[(d + 3) % 4]) as usize];
            let forward = map[(cursor + DIRECTIONS[d]) as usize];
            //println!("{} {} {}", left as char, right as char, forward as char);
            //let mut s = String::new();
            //stdin().read_line(&mut s).unwrap();
            match (left, right, forward) {
                (1, 1, 1) => {
                    continue 'a;
                }
                (0, _, _) | (_, 0, _) => {
                    let score = stating_score - distance;
                    let old_cost = cost_map[cursor as usize];
                    if old_cost < score {
                        deposit.push((score, cursor, d));
                        cost_map[cursor as usize] = score;
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
        match map[(start + DIRECTIONS[d]) as usize] {
            1 => {
                continue 'a;
            }
            _ => {}
        }
        let mut distance = 1;
        loop {
            let cursor = start as i32 + DIRECTIONS[d] * distance;
            if cursor as usize == ((ROWS - 2) * COLUMS) + COLUMS - 2 {
                previous_tiles.push((previous_node, start, d, distance));
                return Some(stating_score - distance);
            }
            //println!("{} {}", cursor, d);
            let left = map[(cursor + DIRECTIONS[(d + 1) % 4]) as usize];
            let right = map[(cursor + DIRECTIONS[(d + 3) % 4]) as usize];
            let forward = map[(cursor + DIRECTIONS[d]) as usize];
            //println!("{} {} {}", left as char, right as char, forward as char);
            //let mut s = String::new();
            //stdin().read_line(&mut s).unwrap();
            match (left, right, forward) {
                (1, 1, 1) => {
                    continue 'a;
                }
                (0, _, _) | (_, 0, _) => {
                    let score = stating_score - distance;
                    let old_cost = cost_map[cursor as usize];
                    if old_cost < score {
                        deposit.push((score, cursor, d, previous_tiles.len()));
                        previous_tiles.push((previous_node, start, d, distance));
                        cost_map[cursor as usize] = score;
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
