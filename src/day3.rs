use std::fmt::Display;

use regex::Regex;

pub fn part1(input: &str) -> impl Display {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let out = regex
        .captures_iter(input)
        .map(|c| u32::from_str_radix(&c[1], 10).unwrap() * u32::from_str_radix(&c[2], 10).unwrap())
        .fold(0, |acc, current| acc + current);
    out
}
pub fn part2(input: &str) -> impl Display {
    let regex = Regex::new(r"((mul\((\d{1,3}),(\d{1,3})\))|(don't\(\)|do\(\)))").unwrap();
    let captures = regex.captures_iter(input);
    let mut enabled = true;
    let mut out = 0;
    for c in captures {
        match &c[0] {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                /*print!(
                    "{}  {}  {}  {}  {}  {}",
                    &c[0], &c[1], &c[2], &c[3], &c[4], &c[5]
                );*/
                if enabled {
                    out += u32::from_str_radix(&c[3], 10).unwrap()
                        * u32::from_str_radix(&c[4], 10).unwrap();
                }
            }
        }
    }
    out
}
