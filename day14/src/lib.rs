use std::collections::HashMap;

pub fn part1(input: &Vec<&str>) -> u64 {
    solve(input, 10)
}

pub fn part2(input: &Vec<&str>) -> u64 {
    solve(input, 40)
}

pub fn solve(input: &Vec<&str>, steps: usize) -> u64 {
    let template = input[0];
    let mut rules = HashMap::new();

    for line in input.iter().skip(1) {
        let mut split = line.split(" -> ");
        rules.insert(split.next().unwrap(), split.next().unwrap());
    }

    let mut pair_count = HashMap::new();
    for i in 0..template.len() - 1 {
        *pair_count
            .entry(template[i..=i + 1].to_owned())
            .or_insert(0u64) += 1;
    }

    for _ in 0..steps {
        let mut new_pair_count = HashMap::new();

        for (pair, count) in pair_count.iter() {
            let new_element = rules[pair.as_str()];

            *new_pair_count
                .entry(pair[0..1].to_owned() + new_element)
                .or_insert(0) += count;

            *new_pair_count
                .entry(new_element.to_owned() + &pair[1..2])
                .or_insert(0) += count;
        }

        pair_count = new_pair_count;
    }

    let mut char_count = HashMap::new();

    for (pair, count) in pair_count.iter() {
        *char_count.entry(pair[0..1].to_owned()).or_insert(0) += count;
        *char_count.entry(pair[1..2].to_owned()).or_insert(0) += count;
    }

    *char_count
        .get_mut(template.chars().last().unwrap().to_string().as_str())
        .unwrap() += 1;

    let char_count: Vec<u64> = char_count.values().cloned().collect();

    (char_count.iter().max().unwrap() - char_count.iter().min().unwrap()) / 2
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 1588)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 2188189693529)
    }
}
