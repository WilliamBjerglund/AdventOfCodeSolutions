macro_rules! advent_of_code {
    (
        $(
            $year:ident => $year_number:literal {
                $(
                    $day:ident => $day_number:literal
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
                        $day::run();
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

advent_of_code!(
    y2016 => 2016 {
        day1 => 1,
        day2 => 2,
        day3 => 3,
    },
);
