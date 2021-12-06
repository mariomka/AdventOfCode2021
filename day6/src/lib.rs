pub fn part1(input: &Vec<usize>) -> usize {
    solve(input, 80)
}

pub fn part2(input: &Vec<usize>) -> usize {
    solve(input, 256)
}

fn solve(input: &Vec<usize>, days: usize) -> usize {
    let mut lanternfish_counter = [0usize; 9];

    for laternfish in input {
        lanternfish_counter[*laternfish] += 1;
    }

    for _ in 0..days {
        lanternfish_counter = [
            lanternfish_counter[1],
            lanternfish_counter[2],
            lanternfish_counter[3],
            lanternfish_counter[4],
            lanternfish_counter[5],
            lanternfish_counter[6],
            lanternfish_counter[7] + lanternfish_counter[0],
            lanternfish_counter[8],
            lanternfish_counter[0],
        ];
    }

    lanternfish_counter.iter().sum()
}

#[cfg(test)]
mod tests {
    use helpers::parse_split_input;

    use super::*;

    fn input<'a>() -> Vec<usize> {
        let input = "3,4,3,1,2";
        parse_split_input(input, ",")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 5934)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 26984457539)
    }
}
