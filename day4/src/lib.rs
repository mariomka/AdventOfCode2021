use std::fmt::{Debug, Formatter};
use std::str::FromStr;

// use helpers::debug;
use regex::Regex;

pub fn part1(input: &Vec<&str>) -> usize {
    let numbers = parse_numbers(input);
    let mut boards = parse_boards(input);
    let mut winner: Option<Board> = None;
    let mut last_number = 0;

    'outer: for number in numbers {
        last_number = number;

        for index in 0..boards.len() {
            let mut board = boards[index].clone();

            board.mark(number);

            if board.is_completed() {
                winner = Some(board.clone());
                break 'outer;
            }

            boards[index] = board;
        }
    }

    let winner = winner.unwrap();
    // debug!(winner);

    winner.score(last_number)
}

pub fn part2(input: &Vec<&str>) -> usize {
    let numbers = parse_numbers(input);
    let mut boards = parse_boards(input);
    let mut loser: Option<Board> = None;
    let mut last_number = 0;
    let mut completed_boards: Vec<usize> = Vec::new();

    'outer: for number in numbers {
        last_number = number;

        for index in 0..boards.len() {
            if completed_boards.contains(&index) {
                continue;
            }

            let mut board = boards[index].clone();

            board.mark(number);

            if board.is_completed() {
                completed_boards.push(index);

                if completed_boards.len() == boards.len() {
                    loser = Some(board.clone());
                    break 'outer;
                }
            }

            boards[index] = board;
        }
    }

    let looser = loser.unwrap();
    // debug!(looser);

    looser.score(last_number)
}

fn parse_numbers(input: &Vec<&str>) -> Vec<usize> {
    input[0]
        .split(",")
        .map(|number| number.parse().unwrap())
        .collect()
}

fn parse_boards(input: &Vec<&str>) -> Vec<Board> {
    return (1..input.len())
        .map(|i| Board::from_str(input[i]).unwrap())
        .collect();
}

#[derive(Clone)]
struct Number {
    number: usize,
    marked: bool,
}

#[derive(Clone)]
struct Board {
    numbers: Vec<Number>,
    marked_counter: usize,
}

impl Board {
    fn mark(&mut self, number: usize) {
        for num in self.numbers.iter_mut() {
            if num.number == number {
                num.marked = true;
                self.marked_counter += 1;
                break;
            }
        }
    }

    fn is_completed(&self) -> bool {
        if 5 > self.marked_counter {
            return false;
        }

        (self.numbers[0].marked
            && self.numbers[1].marked
            && self.numbers[2].marked
            && self.numbers[3].marked
            && self.numbers[4].marked)
            || (self.numbers[5].marked
                && self.numbers[6].marked
                && self.numbers[7].marked
                && self.numbers[8].marked
                && self.numbers[9].marked)
            || (self.numbers[10].marked
                && self.numbers[11].marked
                && self.numbers[12].marked
                && self.numbers[13].marked
                && self.numbers[14].marked)
            || (self.numbers[15].marked
                && self.numbers[16].marked
                && self.numbers[17].marked
                && self.numbers[18].marked
                && self.numbers[19].marked)
            || (self.numbers[20].marked
                && self.numbers[21].marked
                && self.numbers[22].marked
                && self.numbers[23].marked
                && self.numbers[24].marked)
            || (self.numbers[0].marked
                && self.numbers[5].marked
                && self.numbers[10].marked
                && self.numbers[15].marked
                && self.numbers[20].marked)
            || (self.numbers[1].marked
                && self.numbers[6].marked
                && self.numbers[11].marked
                && self.numbers[16].marked
                && self.numbers[21].marked)
            || (self.numbers[2].marked
                && self.numbers[7].marked
                && self.numbers[12].marked
                && self.numbers[17].marked
                && self.numbers[22].marked)
            || (self.numbers[3].marked
                && self.numbers[8].marked
                && self.numbers[13].marked
                && self.numbers[18].marked
                && self.numbers[23].marked)
            || (self.numbers[4].marked
                && self.numbers[9].marked
                && self.numbers[14].marked
                && self.numbers[19].marked
                && self.numbers[24].marked)
    }

    fn score(&self, last_number: usize) -> usize {
        self.numbers.iter().fold(0, |acc, number| {
            if !number.marked {
                acc + number.number
            } else {
                acc
            }
        }) * last_number
    }
}

impl FromStr for Board {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Board {
            numbers: Regex::new(r"\s+")
                .unwrap()
                .split(input)
                .map(|number| Number {
                    number: number.parse().unwrap(),
                    marked: false,
                })
                .collect(),
            marked_counter: 0,
        })
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n")?;

        for (index, number) in self.numbers.iter().enumerate() {
            if number.marked {
                write!(f, "({}) ", number.number)?;
            } else {
                write!(f, " {}  ", number.number)?;
            }

            if 0 == (index + 1) % 5 {
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use helpers::split_input;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
 ";
        split_input(input, "\n\n")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 4512)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 1924)
    }
}
