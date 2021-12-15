use helpers::{input_grid, run, Grid};

fn main() {
    let input: Grid<usize> = input_grid(include_str!("../input.txt"));

    run("part1", || day15::part1(&input));
    run("part2", || day15::part2(&input));
}
