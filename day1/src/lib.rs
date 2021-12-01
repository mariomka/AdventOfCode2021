use itertools::Itertools;

pub fn part1(input: &Vec<usize>) -> usize {
    input
        .iter()
        .tuple_windows()
        .fold(0, |acc, (a, b)| if b > a { acc + 1 } else { acc })
}

pub fn part2(input: &Vec<usize>) -> usize {
    input
        .iter()
        .tuple_windows::<(_, _, _)>()
        .fold(Vec::new(), |mut acc, (a, b, c)| {
            acc.push(a + b + c);
            acc
        })
        .iter()
        .tuple_windows::<(_, _)>()
        .fold(0, |acc, (a, b)| if b > a { acc + 1 } else { acc })
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
