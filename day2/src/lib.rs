use std::str::FromStr;

enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        let mut split = input.split(' ');

        let command = split.next().unwrap();
        let amount = split.next().unwrap().parse().unwrap();

        match command {
            "forward" => Ok(Command::Forward(amount)),
            "down" => Ok(Command::Down(amount)),
            "up" => Ok(Command::Up(amount)),
            _ => Err(()),
        }
    }
}

pub fn part1(input: &Vec<&str>) -> usize {
    let res = input
        .iter()
        .fold((0, 0), |acc, line| match Command::from_str(line).unwrap() {
            Command::Forward(amount) => (acc.0 + amount, acc.1),
            Command::Down(amount) => (acc.0, acc.1 + amount),
            Command::Up(amount) => (acc.0, acc.1 - amount),
        });

    res.0 * res.1
}

pub fn part2(input: &Vec<&str>) -> usize {
    let res = input.iter().fold((0, 0, 0), |acc, line| {
        match Command::from_str(line).unwrap() {
            Command::Forward(amount) => (acc.0 + amount, acc.1 + acc.2 * amount, acc.2),
            Command::Down(amount) => (acc.0, acc.1, acc.2 + amount),
            Command::Up(amount) => (acc.0, acc.1, acc.2 - amount),
        }
    });

    res.0 * res.1
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 150)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 900)
    }
}
