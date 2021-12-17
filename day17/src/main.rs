use helpers::run;

fn main() {
    let input: &str = include_str!("../input.txt");

    run("part1", || day17::part1(&input));
    run("part2", || day17::part2(&input));
}
