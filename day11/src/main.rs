use helpers::{input_grid, run, Grid};

fn main() {
    let input: Grid<usize> = input_grid(include_str!("../input.txt"));

    run("part1", || day11::part1(input.clone()));
    run("part2", || day11::part2(input.clone()));
}
