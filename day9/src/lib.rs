use helpers::{Coord, Grid};

pub fn part1(input: &Grid<usize>) -> usize {
    get_low_points(input)
        .iter()
        .fold(0, |acc, (point_height, _)| acc + 1 + point_height)
}

pub fn part2(input: &Grid<usize>) -> usize {
    let mut basins: Vec<usize> = get_low_points(input)
        .iter()
        .map(|low_point| get_basin_coords(&input, low_point.0, low_point.1).len())
        .collect();

    basins.sort_unstable();
    basins.reverse();

    basins.iter().take(3).product()
}

fn get_low_points(grid: &Grid<usize>) -> Vec<(usize, Coord)> {
    let mut low_points: Vec<(usize, Coord)> = Vec::new();

    for x in 0..grid.size.0 {
        'outer: for y in 0..grid.size.1 {
            let current_height = grid.cell((x, y));

            for (_, neighbor_height) in grid.neighbors((x, y)) {
                if neighbor_height <= current_height {
                    continue 'outer;
                }
            }

            low_points.push((*current_height, (x, y)));
        }
    }

    low_points
}

fn get_basin_coords(grid: &Grid<usize>, current_height: usize, coord: Coord) -> Vec<Coord> {
    let mut positions: Vec<Coord> = vec![coord];

    for (neighbor_coord, neighbor_height) in grid.neighbors(coord) {
        if *neighbor_height < 9 && *neighbor_height > current_height {
            positions.append(&mut get_basin_coords(
                grid,
                *neighbor_height,
                neighbor_coord,
            ));
        }
    }

    positions.sort_unstable();
    positions.dedup();

    positions
}

#[cfg(test)]
mod tests {
    use helpers::input_grid;

    use super::*;

    fn input<'a>() -> Grid<usize> {
        let input = "\
2199943210
3987894921
9856789892
8767896789
9899965678";
        input_grid(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 15)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 1134)
    }
}
