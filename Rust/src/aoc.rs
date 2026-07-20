macro_rules! advent_of_code {
    (
        $(
            $year:ident => $year_number:literal {
                $(
                    $day:ident => $day_number:literal [$run:literal] // <-- Added [$run:literal]
                ),* $(,)?
            }
        ),* $(,)?
    ) => {
        $(
            pub mod $year {
                $(
                    pub mod $day {
                        pub mod part1 {
                            include!(concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/src/",
                                stringify!($year),
                                "/",
                                stringify!($day),
                                "/part1.rs"
                            ));
                        }

                        pub mod part2 {
                            include!(concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/src/",
                                stringify!($year),
                                "/",
                                stringify!($day),
                                "/part2.rs"
                            ));
                        }

                        const INPUT: &str = include_str!(concat!(
                            env!("CARGO_MANIFEST_DIR"),
                            "/src/",
                            stringify!($year),
                            "/",
                            stringify!($day),
                            "/input"
                        ));

                        pub fn run() {
                            let part1_answer = part1::main(INPUT);
                            let part2_answer = part2::main(INPUT);

                            println!(
                                "Year {}, Day {}, Part 1: {}",
                                $year_number,
                                $day_number,
                                part1_answer
                            );

                            println!(
                                "Year {}, Day {}, Part 2: {}",
                                $year_number,
                                $day_number,
                                part2_answer
                            );
                        }
                    }
                )*

                pub fn run() {
                    println!("--- Advent of Code {} ---", $year_number);

                    $(
                        // Only execute the day if the toggle is set to true
                        if $run {
                            $day::run();
                        }
                    )*
                }
            }
        )*

        pub fn run() {
            $(
                $year::run();
            )*
        }
    };
}

// Now you can easily turn individual days true or false!
advent_of_code!(
    y2016 => 2016 {
        day1 => 1 [false],
        day2 => 2 [false],
        day3 => 3 [false],
        day4 => 4 [false],
        day5 => 5 [false],
        day6 => 6 [true],
        day7 => 7 [true],
    },
);
