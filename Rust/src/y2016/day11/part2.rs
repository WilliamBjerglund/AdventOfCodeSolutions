pub fn main(input: &str) -> usize {
    let extra_items = [[b'E', b'G'], [b'E', b'M'], [b'D', b'G'], [b'D', b'M']];

    super::part1::solve_extra_items(input, &extra_items)
}
