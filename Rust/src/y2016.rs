/*
! Simple runner for Advent of Code 2016 solutions.
! Note that to add a new day you just update the `days!` macro invocation at the bottom of this file.
*/

macro_rules! days {
    ($($day:ident => $day_number:literal),* $(,)?) => {
        $(
            pub mod $day {
                pub mod part1;
                pub mod part2;

                const INPUT: &str = include_str!(
                    concat!("y2016/", stringify!($day), "/input")
                );

                pub fn run() {
                    let part1_answer = part1::main(INPUT);
                    let part2_answer = part2::main(INPUT);

                    println!("Day {}, Part 1: {}", $day_number, part1_answer);
                    println!("Day {}, Part 2: {}", $day_number, part2_answer);
                }
            }
        )*

        pub fn run() {
            $(
                $day::run();
            )*
        }
    };
}

days!(
    day1 => 1,
    day2 => 2,
    day3 => 3,
);
