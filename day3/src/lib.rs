pub fn part1(input: &Vec<&str>, size: usize) -> usize {
    let input = parse_input(input);

    let mut ones_counts = vec![0usize; size];

    for number in input.iter() {
        for pos in 0..size {
            if bit_is_one(number, pos) {
                ones_counts[pos] += 1;
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    for (i, ones_count) in ones_counts.iter().enumerate() {
        if *ones_count > input.len() - ones_count {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }

    gamma * epsilon
}

pub fn part2(input: &Vec<&str>, size: usize) -> usize {
    let input = parse_input(input);

    let oxygen_rating = calc_part2(input.clone(), size, true);
    let co2_rating = calc_part2(input.clone(), size, false);

    oxygen_rating * co2_rating
}

fn parse_input(input: &Vec<&str>) -> Vec<usize> {
    return input
        .iter()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect();
}

fn bit_is_one(number: &usize, pos: usize) -> bool {
    0 != number & (1 << pos)
}

fn calc_part2(input: Vec<usize>, size: usize, wants_ones: bool) -> usize {
    let mut input = input;

    for pos in (0..size).rev() {
        if 1 == input.len() {
            break;
        }

        let mut ones_count = 0;

        for number in input.iter() {
            if bit_is_one(number, pos) {
                ones_count += 1;
            }
        }

        let one_is_most_common = ones_count >= input.len() - ones_count;

        input.retain(|number| {
            let is_one = bit_is_one(number, pos);

            if one_is_most_common {
                is_one == wants_ones
            } else {
                is_one != wants_ones
            }
        });
    }

    return input[0];
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input(), 5), 198)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input(), 5), 230)
    }
}
