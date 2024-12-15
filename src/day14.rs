use std::fmt::Display;

use itertools::Chunk;

const COLUMS: u32 = 101;
const ROWS: u32 = 103;

pub fn part1(input: &str) -> impl Display {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for l in input.lines() {
        let mut blocks = l.split_ascii_whitespace();
        let mut position = blocks.next().unwrap().split(',');
        let mut velocities = blocks.next().unwrap().split(',');
        let px = u32::from_str_radix(&position.next().unwrap()[2..], 10).unwrap();
        let py = u32::from_str_radix(position.next().unwrap(), 10).unwrap();
        let vx = (i32::from_str_radix(&velocities.next().unwrap()[2..], 10).unwrap()
            + COLUMS as i32) as u32
            % COLUMS;
        let vy = (i32::from_str_radix(velocities.next().unwrap(), 10).unwrap() + ROWS as i32)
            as u32
            % ROWS;
        let fx = (px + vx * 100) % COLUMS;
        let fy = (py + vy * 100) % ROWS;
        match (fx, fy) {
            (0..50, 0..51) => {
                q1 += 1;
            }
            (51..101, 0..51) => {
                q2 += 1;
            }
            (0..50, 52..103) => {
                q3 += 1;
            }
            (51..101, 52..103) => {
                q4 += 1;
            }
            _ => {}
        }
    }
    q1 * q2 * q3 * q4
}
pub fn part2(input: &str) -> impl Display {
    let mut bots = Vec::new();
    for l in input.lines() {
        let mut blocks = l.split_ascii_whitespace();
        let mut position = blocks.next().unwrap().split(',');
        let mut velocities = blocks.next().unwrap().split(',');
        let px = u32::from_str_radix(&position.next().unwrap()[2..], 10).unwrap();
        let py = u32::from_str_radix(position.next().unwrap(), 10).unwrap();
        let vx = (i32::from_str_radix(&velocities.next().unwrap()[2..], 10).unwrap()
            + COLUMS as i32) as u32
            % COLUMS;
        let vy = (i32::from_str_radix(velocities.next().unwrap(), 10).unwrap() + ROWS as i32)
            as u32
            % ROWS;
        bots.push((px, py, vx, vy));
    }
    let mut step = 0;
    let mut row = 0;
    for i in 0..ROWS {
        let mut occupancy = [0; ROWS as usize];
        for (_x, y, _vx, vy) in bots.iter() {
            let fy1 = (*y + *vy * i) % ROWS;
            occupancy[fy1 as usize] += 1;
        }
        if let Some((r, _)) = occupancy
            .iter()
            .enumerate()
            .filter(|(_, &s)| s >= 16)
            .next()
        {
            step = i;
            row = r;
        }
    }
    bots = bots
        .into_iter()
        .filter(|(_, y, _, vy)| ((*y + *vy * step) % ROWS) as usize == row)
        .collect();
    for i in 0..COLUMS {
        let mut drawing1 = [0; COLUMS as usize];
        for (x, _, vx, _) in bots.iter() {
            let fx1 = (*x + *vx * (i * ROWS + step)) % COLUMS;
            drawing1[fx1 as usize] = 1;
        }
        let mut r1 = 0u16;
        for x in 0..COLUMS {
            r1 = (r1 << 1) + drawing1[x as usize];
            if r1 == u16::MAX {
                return i * ROWS + step;
            }
        }
    }
    /*for i in 0..4096 {
        let mut drawing1 = [0; (COLUMS * ROWS) as usize];
        let mut drawing2 = [0; (COLUMS * ROWS) as usize];
        let mut drawing3 = [0; (COLUMS * ROWS) as usize];
        let mut drawing4 = [0; (COLUMS * ROWS) as usize];
        for (x, y, vx, vy) in bots.iter_mut() {
            let fx1 = (*x + *vx * 1) % COLUMS;
            let fy1 = (*y + *vy * 1) % ROWS;
            let fx2 = (*x + *vx * 2) % COLUMS;
            let fy2 = (*y + *vy * 2) % ROWS;
            let fx3 = (*x + *vx * 3) % COLUMS;
            let fy3 = (*y + *vy * 3) % ROWS;
            let fx4 = (*x + *vx * 4) % COLUMS;
            let fy4 = (*y + *vy * 4) % ROWS;
            drawing1[(fx1 + fy1 * COLUMS) as usize] = 1;
            drawing2[(fx2 + fy2 * COLUMS) as usize] = 1;
            drawing3[(fx3 + fy3 * COLUMS) as usize] = 1;
            drawing4[(fx4 + fy4 * COLUMS) as usize] = 1;
            *x = fx4;
            *y = fy4;
        }
        let mut r1 = 0u16;
        let mut r2 = 0u16;
        let mut r3 = 0u16;
        let mut r4 = 0u16;
        for r in 0..ROWS {
            for x in 0..COLUMS {
                r1 = (r1 << 1) + drawing1[(x + r * COLUMS) as usize];
                r2 = (r2 << 1) + drawing2[(x + r * COLUMS) as usize];
                r3 = (r3 << 1) + drawing3[(x + r * COLUMS) as usize];
                r4 = (r4 << 1) + drawing4[(x + r * COLUMS) as usize];
                if r1 == u16::MAX {
                    return i * 4 + 1;
                }
                if r2 == u16::MAX {
                    return i * 4 + 2;
                }
                if r3 == u16::MAX {
                    return i * 4 + 3;
                }
                if r4 == u16::MAX {
                    return i * 4 + 4;
                }
            }
        }
    }*/
    return u32::MAX;
}
