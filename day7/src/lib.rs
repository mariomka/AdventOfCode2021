pub fn part1(input: &Vec<usize>) -> usize {
    solve(input, |moves| moves)
}

pub fn part2(input: &Vec<usize>) -> usize {
    solve(input, |moves| (moves * moves + moves) / 2)
}

pub fn solve<F>(crab_positions: &Vec<usize>, fuel_calculator: F) -> usize
where
    F: Fn(usize) -> usize,
{
    let mut least_fuel = usize::MAX;

    'outer: for position in 0..=*crab_positions.iter().max().unwrap() {
        let mut fuel = 0;
        for crab_position in crab_positions.iter() {
            let moves = (position as isize - *crab_position as isize).abs() as usize;
            fuel += fuel_calculator(moves);

            if fuel > least_fuel {
                continue 'outer;
            }
        }

        if fuel < least_fuel {
            least_fuel = fuel;
        }
    }

    least_fuel
}

#[cfg(test)]
mod tests {
    use helpers::parse_split_input;

    use super::*;

    fn input() -> Vec<usize> {
        let input = "16,1,2,0,4,2,7,1,2,14";
        parse_split_input(input, ",")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 37)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 168)
    }
}
