use std::{collections::HashSet, fmt::Display};
use wide::u8x16;
use Debug;
const COLUMS: usize = 44;
const ROWS: usize = 44;
const ROWSIZE: usize = 45;
pub fn part2(input: &str) -> impl Display {
    let map = input.as_bytes();
    let mut total_1 = [0u8; ROWS * COLUMS];
    let mut total_2 = [0u8; ROWS * COLUMS];
    for y in 0..ROWS {
        for x in 0..COLUMS {
            total_1[x + y * COLUMS] = if map[x + y * ROWSIZE] == 57 { 1 } else { 0 };
            //print!("{}", total_1[x + y * COLUMS]);
        }
    }
    //println!("");
    for i in (0..9).into_iter() {
        let (origin, destination) = if i % 2 == 0 {
            (&total_1, &mut total_2)
        } else {
            (&total_2, &mut total_1)
        };
        for y in 0..ROWS {
            for x in 0..COLUMS {
                destination[x + y * COLUMS] = if map[x + y * ROWSIZE] == 56 - i as u8 {
                    let mut sum = 0;
                    if x > 0 {
                        sum += origin[x - 1 + y * COLUMS]
                    }
                    if x < COLUMS - 1 {
                        sum += origin[x + 1 + y * COLUMS]
                    }
                    if y < ROWS - 1 {
                        sum += origin[x + (y + 1) * COLUMS]
                    }
                    if y > 0 {
                        sum += origin[x + (y - 1) * COLUMS]
                    }

                    sum
                } else {
                    0
                };
                //print!("{}", destination[x + y * COLUMS]);
            }
        }
        //println!("");
    }
    total_2.iter().fold(0, |acc, i| acc + (*i as u32))
}
pub fn part1(input: &str) -> impl Display {
    let map = input.as_bytes();
    let mut reachable_1: Vec<HashSet<usize>> = vec![HashSet::new(); ROWS * COLUMS];
    let mut reachable_2: Vec<HashSet<usize>> = vec![HashSet::new(); ROWS * COLUMS];
    for y in 0..ROWS {
        for x in 0..COLUMS {
            if map[x + y * ROWSIZE] == 57 {
                reachable_1[x + y * COLUMS].insert(x + y * COLUMS);
            }
        }
    }
    for i in (0..9).into_iter() {
        let (origin, destination) = if i % 2 == 0 {
            (&reachable_1, &mut reachable_2)
        } else {
            (&reachable_2, &mut reachable_1)
        };
        for y in 0..ROWS {
            for x in 0..COLUMS {
                let sum = &mut destination[x + y * COLUMS];
                if map[x + y * ROWSIZE] == 56 - i as u8 {
                    if x > 0 {
                        sum.extend(&origin[x - 1 + y * COLUMS]);
                    }
                    if x < COLUMS - 1 {
                        sum.extend(&origin[x + 1 + y * COLUMS]);
                    }
                    if y < ROWS - 1 {
                        sum.extend(&origin[x + (y + 1) * COLUMS]);
                    }
                    if y > 0 {
                        sum.extend(&origin[x + (y - 1) * COLUMS]);
                    }
                } else {
                    sum.clear();
                };
            }
        }
    }
    reachable_2.iter().fold(0, |acc, i| acc + i.len())
}

pub fn check_simd(map: &[u8], index: usize, depht: u8) -> (u8x16, u8) {
    let check = u8x16::from(depht);
    let to_chech = u8x16::from(&map[index..index]);
    let out = to_chech.cmp_eq(check) & u8x16::ONE;
    (out, out.as_array_ref().iter().fold(0, |acc, i| acc + i))
}
