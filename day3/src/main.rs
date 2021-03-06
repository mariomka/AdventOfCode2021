use helpers::{input_lines, run};

fn main() {
    let input: Vec<&str> = input_lines(include_str!("../input.txt"));

    run("part1", || day3::part1(&input, 12));
    run("part2", || day3::part2(&input, 12));
}
