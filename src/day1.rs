use std::fmt::Display;

pub fn part1(input: &str) -> impl Display {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    for line in input.lines() {
        let mut row = line.split_ascii_whitespace();
        vec1.push(row.next().unwrap().parse::<i32>().unwrap());
        vec2.push(row.next().unwrap().parse::<i32>().unwrap());
    }
    vec1.sort();
    vec2.sort();
    let mut out = 0;
    for i in 0..vec1.len() {
        out += (vec1[i] - vec2[i]).abs();
    }
    out
}
pub fn part2(input: &str) -> impl Display {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    for line in input.lines() {
        let mut row = line.split_ascii_whitespace();
        vec1.push(row.next().unwrap().parse::<i32>().unwrap());
        vec2.push(row.next().unwrap().parse::<i32>().unwrap());
    }
    vec1.sort();
    vec2.sort();
    let mut out = 0;
    let mut progress = 0;
    for left in vec1 {
        while vec2[progress] < left {
            progress += 1;
        }
        while vec2[progress] == left {
            out += left;
            progress += 1;
        }
    }
    out
}
