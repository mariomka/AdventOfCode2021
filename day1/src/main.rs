use helpers::{parse_input, run};

fn main() {
    let input: Vec<usize> = parse_input(include_str!("../input.txt"));

    run("part1", || day1::part1(&input));
    run("part2", || day1::part2(&input));
}
