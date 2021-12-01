pub fn part1(input: &Vec<usize>) -> usize {
    input.windows(2).filter(|w| w[1] > w[0]).count()
}

pub fn part2(input: &Vec<usize>) -> usize {
    input.windows(4).filter(|w| w[3] > w[0]).count()
}

#[cfg(test)]
mod tests {
    use helpers::parse_input;

    use super::*;

    fn input() -> Vec<usize> {
        let input = "\
199
200
208
210
200
207
240
269
260
263";
        parse_input(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 7)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 5)
    }
}
