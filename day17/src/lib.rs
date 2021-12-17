use regex::Regex;

pub fn part1(input: &str) -> isize {
    let (_, p2) = parse(input);

    return p2.y * (p2.y + 1) / 2;
}

pub fn part2(input: &str) -> isize {
    let (p1, p2) = parse(input);
    let mut matches = 0;

    let x_min = ((((8 * p1.x + 1) as f64).sqrt() - 1.0) / 2.0).floor() as isize;
    let x_max = p2.x;

    let y_min = p2.y;
    let y_max = p2.y.abs() - 1;

    for x in x_min..=x_max {
        for y in y_min..=y_max {
            let mut velocity = (x, y);
            let mut pos = (0isize, 0isize);

            loop {
                pos.0 += velocity.0;
                pos.1 += velocity.1;

                if pos.0 >= p1.x && pos.0 <= p2.x && pos.1 <= p1.y && pos.1 >= p2.y {
                    matches += 1;
                    break;
                } else if pos.0 > p2.x || pos.1 < p2.y {
                    break;
                }

                velocity.0 += if velocity.0 > 0 { -1 } else { 0 };
                velocity.1 -= 1;
            }
        }
    }

    matches
}

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

fn parse(input: &str) -> (Point, Point) {
    let regex = Regex::new(
        r"^target area: x=(?P<x1>-?\d+)\.\.(?P<x2>-?\d+), y=(?P<y2>-?\d+)\.\.(?P<y1>-?\d+)$",
    )
    .unwrap();
    let captures = regex.captures(input).unwrap();

    let x1: isize = captures.name("x1").unwrap().as_str().parse().unwrap();
    let x2: isize = captures.name("x2").unwrap().as_str().parse().unwrap();
    let y1: isize = captures.name("y1").unwrap().as_str().parse().unwrap();
    let y2: isize = captures.name("y2").unwrap().as_str().parse().unwrap();

    (Point { x: x1, y: y1 }, Point { x: x2, y: y2 })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "target area: x=20..30, y=-10..-5"
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 45)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 112)
    }
}
