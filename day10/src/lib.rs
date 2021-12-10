use phf::{phf_map, Map};

static PAIRS: Map<char, char> = phf_map! {
    '(' => ')',
    '[' => ']',
    '{' => '}',
    '<' => '>',
};

pub fn part1(input: &Vec<&str>) -> usize {
    input
        .iter()
        .map(|line| match find_error(line) {
            Err(char) => match char {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!("Unexpected character"),
            },
            _ => 0,
        })
        .sum()
}

pub fn part2(input: &Vec<&str>) -> usize {
    let mut scores: Vec<usize> = input
        .iter()
        .map(|line| match find_error(line) {
            Err(_) => 0,
            Ok(stack) => stack.iter().rev().fold(0, |acc, char| {
                acc * 5
                    + match char {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!("unexpected character"),
                    }
            }),
        })
        .filter(|score| *score > 0)
        .collect();

    scores.sort_unstable();

    scores[scores.len() / 2]
}

fn find_error(line: &str) -> Result<Vec<char>, char> {
    let mut stack = Vec::new();

    for char in line.chars() {
        if PAIRS.contains_key(&char) {
            stack.push(char);
        } else if char != PAIRS[&stack.pop().unwrap()] {
            return Err(char);
        }
    }

    Ok(stack)
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 26397)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 288957)
    }
}
