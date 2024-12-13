use std::error::Error;
use std::{
    cmp,
    collections::{hash_map, HashMap},
    fmt::Display,
};

pub fn part1(input: &str) -> impl Display {
    let mut machine_cost_map: HashMap<(u64, u64), Option<u64>> = HashMap::new();
    let mut out = 0;
    let mut lines = input.lines();
    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        let mut line1 = line1.split(",");
        let ax = u64::from_str_radix(&line1.next().unwrap()[12..], 10).unwrap();
        let ay = u64::from_str_radix(&line1.next().unwrap()[3..], 10).unwrap();
        let mut line2 = line2.split(",");
        let bx = u64::from_str_radix(&line2.next().unwrap()[12..], 10).unwrap();
        let by = u64::from_str_radix(&line2.next().unwrap()[3..], 10).unwrap();
        let mut line3 = line3.split(",");
        let gx = u64::from_str_radix(&line3.next().unwrap()[9..], 10).unwrap();
        let gy = u64::from_str_radix(&line3.next().unwrap()[3..], 10).unwrap();
        if let Some(cost) = search_linear(
            //&mut machine_cost_map,
            ax as i64, ay as i64, bx as i64, by as i64, gx as i64, gy as i64,
        ) {
            //println!("{}", cost);
            out += cost;
        }
        machine_cost_map.clear();
        lines.next();
    }
    out
}
pub fn part2(input: &str) -> impl Display {
    let mut machine_cost_map: HashMap<(u64, u64), Option<u64>> = HashMap::new();
    let mut out = 0;
    let mut lines = input.lines();
    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        let mut line1 = line1.split(",");
        let ax = u64::from_str_radix(&line1.next().unwrap()[12..], 10).unwrap();
        let ay = u64::from_str_radix(&line1.next().unwrap()[3..], 10).unwrap();
        let mut line2 = line2.split(",");
        let bx = u64::from_str_radix(&line2.next().unwrap()[12..], 10).unwrap();
        let by = u64::from_str_radix(&line2.next().unwrap()[3..], 10).unwrap();
        let mut line3 = line3.split(",");
        let gx = u64::from_str_radix(&line3.next().unwrap()[9..], 10).unwrap();
        let gy = u64::from_str_radix(&line3.next().unwrap()[3..], 10).unwrap();

        if let Some(cost) = search_linear(
            //&mut machine_cost_map,
            ax as i64,
            ay as i64,
            bx as i64,
            by as i64,
            gx as i64 + 10000000000000,
            gy as i64 + 10000000000000,
        ) {
            //println!("{}", cost);
            out += cost;
        }
        machine_cost_map.clear();
        lines.next();
    }
    out
}

fn search(
    machine_cost_map: &mut HashMap<(u64, u64), Option<u64>>,
    ax: u64,
    ay: u64,
    bx: u64,
    by: u64,
    gx: u64,
    gy: u64,
    depth: u64,
    current_x: u64,
    current_y: u64,
) -> Option<u64> {
    /*if depth == 200 {
        return None;
    }*/
    if current_x > gx || current_y > gy {
        return None;
    }
    if current_x == gx && current_y == gy {
        return Some(0);
    }
    if let Some(cost) = machine_cost_map.get(&(current_x, current_y)) {
        return *cost;
    };
    let c1 = search(
        machine_cost_map,
        ax,
        ay,
        bx,
        by,
        gx,
        gy,
        depth + 1,
        current_x + ax,
        current_y + ay,
    );
    let c2 = search(
        machine_cost_map,
        ax,
        ay,
        bx,
        by,
        gx,
        gy,
        depth + 1,
        current_x + bx,
        current_y + by,
    );
    let out = match (c1, c2) {
        (Some(x), None) => Some(x + 3),
        (None, Some(x)) => Some(x + 1),
        (Some(x), Some(y)) => Some(cmp::min(x + 3, y + 1)),
        (None, None) => None,
    };
    machine_cost_map.insert((current_x, current_y), out);
    out
}
fn search_iter(
    machine_cost_map: &mut HashMap<(u64, u64), Option<u64>>,
    ax: u64,
    ay: u64,
    bx: u64,
    by: u64,
    gx: u64,
    gy: u64,
) -> Option<u64> {
    /*if depth == 200 {
        return None;
    }*/
    machine_cost_map.insert((gx, gy), Some(0));
    let mut requests_stack = Vec::new();
    requests_stack.push((0, 0));
    while let Some((current_x, current_y)) = requests_stack.pop() {
        if current_x > gx || current_y > gy {
            machine_cost_map.insert((current_x, current_y), None);
        }
        let Some(c1) = machine_cost_map.get(&(current_x + ax, current_y + ay)) else {
            requests_stack.push((current_x, current_y));
            requests_stack.push((current_x + ax, current_y + ay));
            continue;
        };
        let Some(c2) = machine_cost_map.get(&(current_x + bx, current_y + by)) else {
            requests_stack.push((current_x, current_y));
            requests_stack.push((current_x + bx, current_y + by));
            continue;
        };
        let out = match (c1, c2) {
            (Some(x), None) => Some(x + 3),
            (None, Some(x)) => Some(x + 1),
            (Some(x), Some(y)) => Some(cmp::min(x + 3, y + 1)),
            (None, None) => None,
        };
        machine_cost_map.insert((current_x, current_y), out);
    }
    return *machine_cost_map.get(&(0, 0)).unwrap();
}
fn search_linear(ax: i64, ay: i64, bx: i64, by: i64, gx: i64, gy: i64) -> Option<u64> {
    // a*ax+b*bx=gx
    // a*ay+b*by=gy
    let det = ax * by - ay * bx;
    if det == 0 {
        return None;
    }
    let a = (gx * by - gy * bx) / det;
    let b = (ax * gy - ay * gx) / det;
    if (ax * a + bx * b, ay * a + by * b) == (gx, gy) {
        Some((a * 3 + b) as u64)
    } else {
        None
    }
}
