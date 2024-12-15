use std::fs;

fn main() {
    println!(
        "{}",
        advent_of_codspeed::day14::part1(
            fs::read_to_string("devinput.txt")
                .expect("Error in reading the file")
                .as_str()
        )
    );
    println!(
        "{}",
        advent_of_codspeed::day14::part2(
            fs::read_to_string("devinput.txt")
                .expect("Error in reading the file")
                .as_str()
        )
    );
}
