use std::collections::HashMap;

use regex::Regex;

pub fn part1(input: &Vec<&str>) -> usize {
    solve(input, false)
}

pub fn part2(input: &Vec<&str>) -> usize {
    solve(input, true)
}

fn solve(input: &Vec<&str>, with_diagonals: bool) -> usize {
    let segments = parse_input(input, with_diagonals);
    let mut points: HashMap<Point, usize> = HashMap::new();

    for segment in segments {
        for point in segment {
            points
                .entry(point)
                .and_modify(|entry| *entry += 1)
                .or_insert(1);
        }
    }

    points
        .values()
        .fold(0, |acc, count| if *count > 1usize { acc + 1 } else { acc })
}

fn parse_input(input: &Vec<&str>, with_diagonals: bool) -> Vec<Segment> {
    let mut segments = Vec::new();
    let regex = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();

    for line in input {
        let captures = regex.captures(line).unwrap();

        segments.push(Segment::new(
            Point {
                x: captures.name("x1").unwrap().as_str().parse().unwrap(),
                y: captures.name("y1").unwrap().as_str().parse().unwrap(),
            },
            Point {
                x: captures.name("x2").unwrap().as_str().parse().unwrap(),
                y: captures.name("y2").unwrap().as_str().parse().unwrap(),
            },
            with_diagonals,
        ))
    }

    segments
}

#[derive(Debug, Clone)]
struct Segment {
    p1: Point,
    p2: Point,
    increment: Option<Point>,
}

impl Segment {
    fn new(p1: Point, p2: Point, with_diagonals: bool) -> Self {
        let is_horizontal = p1.y == p2.y;
        let is_vertical = !is_horizontal && p1.x == p2.x;
        let is_diagonal = !is_vertical
            && (p1.x as isize - p2.x as isize).abs() == (p1.y as isize - p2.y as isize).abs();

        let increment = if is_horizontal {
            Some(Point {
                x: if p1.x < p2.x { 1 } else { -1 },
                y: 0,
            })
        } else if is_vertical {
            Some(Point {
                x: 0,
                y: if p1.y < p2.y { 1 } else { -1 },
            })
        } else if with_diagonals && is_diagonal {
            Some(Point {
                x: if p1.x < p2.x { 1 } else { -1 },
                y: if p1.y < p2.y { 1 } else { -1 },
            })
        } else {
            None
        };

        Segment { p1, p2, increment }
    }
}

impl IntoIterator for Segment {
    type Item = Point;
    type IntoIter = SegmentIterator;

    fn into_iter(self) -> Self::IntoIter {
        SegmentIterator {
            segment: self,
            index: 0,
            finished: false,
        }
    }
}

struct SegmentIterator {
    segment: Segment,
    index: isize,
    finished: bool,
}

impl Iterator for SegmentIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished || self.segment.increment.is_none() {
            self.finished = true;

            return None;
        }

        let increment = self.segment.increment.as_ref().unwrap();
        let point = Point {
            x: self.segment.p1.x + self.index * increment.x,
            y: self.segment.p1.y + self.index * increment.y,
        };

        if point.x == self.segment.p2.x && point.y == self.segment.p2.y {
            self.finished = true;
        }

        self.index += 1;

        Some(point)
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 5)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 12)
    }
}
