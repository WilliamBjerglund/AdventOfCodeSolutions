#[path = "2016/day1/part1.rs"]
mod part1;

#[path = "2016/day1/part2.rs"]
mod part2;

fn main() {
    let input = include_str!("2016/day1/input.txt");

    let answer1 = part1::main(input);
    let answer2 = part2::main(input);

    println!("Day 1, Part 1: {answer1}");
    println!("Day 1, Part 2: {answer2}");
}
