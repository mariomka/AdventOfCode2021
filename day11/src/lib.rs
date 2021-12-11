use helpers::{Coord, Grid};

pub fn part1(input: Grid<usize>) -> usize {
    let mut grid: Grid<usize> = input;
    let mut flashes = 0;

    for _ in 0..100 {
        flashes += step(&mut grid);
    }

    flashes
}

pub fn part2(input: Grid<usize>) -> usize {
    let mut grid: Grid<usize> = input;
    let mut steps = 0;

    loop {
        let flashes = step(&mut grid);
        steps += 1;

        if flashes == grid.len() {
            return steps;
        }
    }
}

fn step(grid: &mut Grid<usize>) -> usize {
    grid.iter_mut().for_each(|(_, energy)| *energy += 1);

    apply_flashes(grid);

    let flashed: Vec<(Coord, usize)> = grid.into_iter().filter(|(_, energy)| *energy > 9).collect();
    let flashes = flashed.len();

    for (coord, _) in flashed.into_iter() {
        grid.set(coord, 0);
    }

    flashes
}

fn apply_flashes(grid: &mut Grid<usize>) {
    let will_flash: Vec<(Coord, usize)> = grid
        .into_iter()
        .filter(|(_, energy)| *energy > 9 && *energy < 100)
        .collect();

    if will_flash.len() == 0 {
        return;
    }

    for (coord, _) in will_flash.into_iter() {
        for (neighbor_coord, energy) in grid
            .neighbors_iter(coord, true)
            .collect::<Vec<(Coord, usize)>>()
        {
            grid.set(neighbor_coord, energy + 1);
        }

        grid.set(coord, 100);
    }

    return apply_flashes(grid);
}

#[cfg(test)]
mod tests {
    use helpers::{input_grid, Grid};

    use super::*;

    fn input<'a>() -> Grid<usize> {
        let input = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        input_grid(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1656)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 195)
    }
}
