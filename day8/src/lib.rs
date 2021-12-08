use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub fn part1(input: &Vec<&str>) -> usize {
    input
        .iter()
        .map(|line| line.split(" | ").collect::<Vec<&str>>()[1])
        .fold(0, |acc, output| {
            acc + output
                .split(" ")
                .filter(|segment| {
                    segment.len() == 2
                        || segment.len() == 4
                        || segment.len() == 3
                        || segment.len() == 7
                })
                .count()
        })
}

pub fn part2(lines: &Vec<&str>) -> usize {
    let mut total_output = 0;

    for line in lines {
        let mut line_split = line.split(" | ");
        let entry_input: Vec<&str> = line_split.next().unwrap().split_whitespace().collect();
        let entry_output: Vec<&str> = line_split.next().unwrap().split_whitespace().collect();
        drop(line_split);

        let mut numbers: Vec<Option<HashSet<char>>> = vec![None; 10];
        let mut numbers_by_segments: HashMap<usize, Vec<HashSet<char>>> = HashMap::new();

        for input in entry_input {
            let chars = HashSet::from_iter(input.chars());

            match input.len() {
                2 => {
                    numbers[1] = Some(chars);
                }
                4 => {
                    numbers[4] = Some(chars);
                }
                3 => {
                    numbers[7] = Some(chars);
                }
                7 => {
                    numbers[8] = Some(chars);
                }
                _ => {
                    numbers_by_segments
                        .entry(input.len())
                        .and_modify(|entry| entry.push(chars.clone()))
                        .or_insert_with(|| {
                            let mut vec = Vec::new();
                            vec.push(chars.clone());
                            vec
                        });
                }
            }
        }

        let four_seven_diff: HashSet<char> = numbers[4]
            .as_ref()
            .unwrap()
            .symmetric_difference(&numbers[7].as_ref().unwrap())
            .cloned()
            .collect();

        for zero_six_or_nine in numbers_by_segments.get(&6).unwrap() {
            if 4 == zero_six_or_nine
                .intersection(numbers[4].as_ref().unwrap())
                .count()
            {
                numbers[9] = Some(zero_six_or_nine.to_owned());
            } else if 3 == zero_six_or_nine.intersection(&four_seven_diff).count() {
                numbers[6] = Some(zero_six_or_nine.to_owned());
            } else {
                numbers[0] = Some(zero_six_or_nine.to_owned());
            }
        }

        for two_three_or_five in numbers_by_segments.get(&5).unwrap() {
            if 4 == two_three_or_five
                .intersection(numbers[9].as_ref().unwrap())
                .count()
            {
                numbers[2] = Some(two_three_or_five.to_owned());
            } else if 2
                == two_three_or_five
                    .intersection(numbers[1].as_ref().unwrap())
                    .count()
            {
                numbers[3] = Some(two_three_or_five.to_owned());
            } else {
                numbers[5] = Some(two_three_or_five.to_owned());
            }
        }

        let numbers: Vec<HashSet<char>> =
            numbers.into_iter().map(|number| number.unwrap()).collect();

        let mut output_number = 0;

        for (index, output) in entry_output.iter().rev().enumerate() {
            let output_chars = HashSet::from_iter(output.chars());

            for (number, chars) in numbers.iter().enumerate() {
                if 0 == output_chars.symmetric_difference(&chars).count() {
                    output_number += number * 10_usize.pow(index as u32);
                    break;
                }
            }
        }

        total_output += output_number;
    }

    total_output
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 26)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 61229)
    }
}
