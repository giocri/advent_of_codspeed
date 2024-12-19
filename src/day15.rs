use std::{collections::VecDeque, fmt::Display};
const COLUMS: usize = 50;
const ROWS: usize = 50;
const ROWSIZE: usize = 51;
const COLUMS_P2: usize = 100;

pub fn part1(input: &str) -> impl Display {
    let input = input.as_bytes();
    let mut map = [0u8; ROWS * ROWSIZE];
    map.copy_from_slice(&input[0..ROWS * ROWSIZE]);

    let index = map.iter().position(|&a| a == b'@').unwrap();
    let mut cursor_x = index % ROWSIZE;
    let mut cursor_y = index / ROWSIZE;
    let moves = &input[ROWS * ROWSIZE..];
    'a: for m in moves {
        match *m {
            b'<' => match map[ROWSIZE * cursor_y + cursor_x - 1] {
                b'#' => {
                    continue 'a;
                }
                b'O' => {
                    'b: for i in (0..cursor_x - 1).into_iter().rev() {
                        let current = map[ROWSIZE * cursor_y + i];
                        if current == b'O' {
                            continue 'b;
                        }
                        if current != b'#' {
                            map[ROWSIZE * cursor_y + i] = b'O';
                            map[ROWSIZE * cursor_y + cursor_x - 1] = b'.';
                            cursor_x -= 1;
                        }
                        break;
                    }
                }
                _ => {
                    cursor_x -= 1;
                }
            },

            b'>' => match map[ROWSIZE * cursor_y + cursor_x + 1] {
                b'#' => {
                    continue 'a;
                }
                b'O' => {
                    'b: for i in (cursor_x + 2)..COLUMS {
                        let current = map[ROWSIZE * cursor_y + i];
                        if current == b'O' {
                            continue 'b;
                        }
                        if current != b'#' {
                            map[ROWSIZE * cursor_y + i] = b'O';
                            map[ROWSIZE * cursor_y + cursor_x + 1] = b'.';
                            cursor_x += 1;
                        }
                        break;
                    }
                }
                _ => {
                    cursor_x += 1;
                }
            },
            b'^' => match map[ROWSIZE * (cursor_y - 1) + cursor_x] {
                b'#' => {
                    continue 'a;
                }
                b'O' => {
                    'b: for i in (0..cursor_y - 1).into_iter().rev() {
                        let current = map[ROWSIZE * i + cursor_x];
                        if current == b'O' {
                            continue 'b;
                        }
                        if current != b'#' {
                            map[ROWSIZE * i + cursor_x] = b'O';
                            map[ROWSIZE * (cursor_y - 1) + cursor_x] = b'.';
                            cursor_y -= 1;
                        }
                        break;
                    }
                }
                _ => {
                    cursor_y -= 1;
                }
            },
            b'v' => match map[ROWSIZE * (cursor_y + 1) + cursor_x] {
                b'#' => {
                    continue 'a;
                }
                b'O' => {
                    'b: for i in (cursor_y + 2)..ROWS {
                        let current = map[ROWSIZE * i + cursor_x];
                        if current == b'O' {
                            continue 'b;
                        }
                        if current != b'#' {
                            map[ROWSIZE * i + cursor_x] = b'O';
                            map[ROWSIZE * (cursor_y + 1) + cursor_x] = b'.';
                            cursor_y += 1;
                        }
                        break;
                    }
                }
                _ => {
                    cursor_y += 1;
                }
            },
            _ => {
                continue;
            }
        }
    }
    /*for r in 0..ROWS {
        for c in 0..COLUMS {
            print!("{}", map[ROWSIZE * r + c] as char)
        }
        println!()
    }*/
    map.iter()
        .enumerate()
        .filter(|(_, &c)| c == b'O')
        .map(|(index, _)| (index / ROWSIZE) * 100 + index % ROWSIZE)
        .fold(0, |acc, x| acc + x)
}
pub fn part2(input: &str) -> impl Display {
    let input = input.as_bytes();
    let mut map = [0u8; ROWS * COLUMS_P2];
    let mut cursor_x = 0;
    let mut cursor_y = 0;
    for r in 0..ROWS {
        for c in 0..COLUMS {
            let simble = input[r * ROWSIZE + c];
            map[(r * COLUMS_P2 + c * 2)..(r * COLUMS_P2 + c * 2 + 2)].copy_from_slice(
                match simble {
                    b'@' => {
                        cursor_x = c * 2;
                        cursor_y = r;
                        &[b'.', b'.']
                    }
                    b'O' => &[b'[', b']'],
                    b'#' => &[b'#', b'#'],
                    _ => &[b'.', b'.'],
                },
            );
        }
    }
    let moves = &input[ROWS * ROWSIZE..];
    let mut to_push = Vec::new();
    'a: for m in moves {
        match *m {
            b'<' => match map[COLUMS_P2 * cursor_y + cursor_x - 1] {
                b'#' => {
                    continue 'a;
                }
                b']' => {
                    to_push.push(COLUMS_P2 * cursor_y + cursor_x - 2);
                    'b: for i in (0..cursor_x - 2).into_iter().rev() {
                        let current = map[COLUMS_P2 * cursor_y + i];
                        if current == b'[' {
                            to_push.push(COLUMS_P2 * cursor_y + i);
                            continue 'b;
                        }
                        if current == b']' {
                            continue 'b;
                        }
                        if current == b'#' {
                            to_push.clear();
                            break 'b;
                        }
                        if current == b'.' {
                            move_boxes(&to_push, -1, &mut map);
                            to_push.clear();
                            cursor_x -= 1;
                            break 'b;
                        }
                    }
                }
                _ => {
                    cursor_x -= 1;
                }
            },

            b'>' => match map[COLUMS_P2 * cursor_y + cursor_x + 1] {
                b'#' => {
                    continue 'a;
                }
                b'[' => {
                    to_push.push(COLUMS_P2 * cursor_y + cursor_x + 1);
                    'b: for i in (cursor_x + 2)..COLUMS_P2 {
                        let current = map[COLUMS_P2 * cursor_y + i];
                        if current == b']' {
                            continue 'b;
                        }
                        if current == b'[' {
                            to_push.push(COLUMS_P2 * cursor_y + i);
                            continue 'b;
                        }
                        if current == b'#' {
                            to_push.clear();
                            break 'b;
                        }
                        if current == b'.' {
                            move_boxes(&to_push, 1, &mut map);
                            to_push.clear();
                            cursor_x += 1;
                            break 'b;
                        }
                    }
                }
                _ => {
                    cursor_x += 1;
                }
            },
            b'^' => match map[COLUMS_P2 * (cursor_y - 1) + cursor_x] {
                b'#' => {
                    continue 'a;
                }
                b']' | b'[' => {
                    if chech_movable(
                        &mut to_push,
                        COLUMS_P2 * (cursor_y) + cursor_x,
                        -(COLUMS_P2 as i32),
                        &mut map,
                    ) {
                        cursor_y -= 1;
                        move_boxes(&to_push, -(COLUMS_P2 as i32), &mut map);
                        to_push.clear();
                    }
                }
                _ => {
                    cursor_y -= 1;
                }
            },
            b'v' => match map[COLUMS_P2 * (cursor_y + 1) + cursor_x] {
                b'#' => {
                    continue 'a;
                }
                b']' | b'[' => {
                    if chech_movable(
                        &mut to_push,
                        COLUMS_P2 * (cursor_y) + cursor_x,
                        COLUMS_P2 as i32,
                        &mut map,
                    ) {
                        cursor_y += 1;
                        move_boxes(&to_push, COLUMS_P2 as i32, &mut map);
                        to_push.clear();
                    }
                }
                _ => {
                    cursor_y += 1;
                }
            },
            _ => {
                continue;
            }
        }
        /* println!("{}", *m as char);
        for r in 0..ROWS {
            for c in 0..COLUMS_P2 {
                if (r == cursor_y && c == cursor_x) {
                    print!("@");
                    continue;
                }
                print!("{}", map[COLUMS_P2 * r + c] as char)
            }
            println!()
        }
        println!("-----------");
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap(); */
    }

    map.iter()
        .enumerate()
        .filter(|(_, &c)| c == b'[')
        .map(|(index, _)| (index / COLUMS_P2) * 100 + index % COLUMS_P2)
        .fold(0, |acc, x| acc + x)
}
fn move_boxes(positions: &[usize], offset: i32, map: &mut [u8]) {
    for &p in positions.iter().rev() {
        (map[p], map[p + 1]) = (b'.', b'.');
        (
            map[(p as i32 + offset) as usize],
            map[(p as i32 + offset) as usize + 1],
        ) = (b'[', b']');
    }
}
fn chech_movable(to_move: &mut Vec<usize>, first: usize, offset: i32, map: &mut [u8]) -> bool {
    //println!("first: {}", first);
    let mut queue = VecDeque::new();
    match map[(first as i32 + offset) as usize] {
        b'#' => {
            to_move.clear();
            return false;
        }
        b']' => {
            let next = (first as i32 - 1 + offset) as usize;
            to_move.push(next);
            queue.push_back(next);
        }
        b'[' => {
            let next = (first as i32 + offset) as usize;
            to_move.push(next);
            queue.push_back(next);
        }
        _ => {}
    }
    //queue.push_back(first);
    //to_move.push(first);
    while let Some(x) = queue.pop_front() {
        let (left, right) = (
            map[(x as i32 + offset) as usize],
            map[(x as i32 + offset + 1) as usize],
        );
        //println!("x: {} left {} right {}", x, left as char, right as char);
        match (left, right) {
            (b'#', _) => {
                to_move.clear();
                return false;
            }
            (_, b'#') => {
                to_move.clear();
                return false;
            }
            (b']', b'.') => {
                let next = (x as i32 - 1 + offset) as usize;
                to_move.push(next);
                queue.push_back(next);
            }
            (b'.', b'[') => {
                let next = (x as i32 + 1 + offset) as usize;
                to_move.push(next);
                queue.push_back(next);
            }
            (b']', b'[') => {
                let next = (x as i32 - 1 + offset) as usize;
                to_move.push(next);
                queue.push_back(next);
                let next = (x as i32 + 1 + offset) as usize;
                to_move.push(next);
                queue.push_back(next);
            }
            (b'[', b']') => {
                let next = (x as i32 + offset) as usize;
                to_move.push(next);
                queue.push_back(next);
            }
            _ => {}
        }
    }
    return true;
}
