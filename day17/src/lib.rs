use regex::Regex;

pub fn part1(input: &str) -> isize {
    let (_, p2) = parse(input);

    p2.y * (p2.y + 1) / 2
}

pub fn part2(input: &str) -> isize {
    let (p1, p2) = parse(input);
    let mut matches = 0;

    let x_min = ((((8 * p1.x + 1) as f64).sqrt() - 1.0) / 2.0).floor() as isize;
    let x_max = p2.x;

    let y_min = p2.y;
    let y_max = p2.y.abs() - 1;

    let area_x = p1.x..=p2.x;
    let area_y = p2.y..=p1.y;

    for x in x_min..=x_max {
        for y in y_min..=y_max {
            let mut velocity = Coord { x, y };
            let mut pos = Coord { x: 0, y: 0 };

            loop {
                pos.x += velocity.x;
                pos.y += velocity.y;

                if area_x.contains(&pos.x) && area_y.contains(&pos.y) {
                    matches += 1;
                    break;
                }

                if pos.x > x_max || pos.y < y_min {
                    break;
                }

                velocity.x += if velocity.x > 0 { -1 } else { 0 };
                velocity.y -= 1;
            }
        }
    }

    matches
}

struct Coord {
    x: isize,
    y: isize,
}

fn parse(input: &str) -> (Coord, Coord) {
    let regex = Regex::new(
        r"^target area: x=(?P<x1>-?\d+)\.\.(?P<x2>-?\d+), y=(?P<y2>-?\d+)\.\.(?P<y1>-?\d+)$",
    )
    .unwrap();
    let captures = regex.captures(input).unwrap();

    let x1: isize = captures.name("x1").unwrap().as_str().parse().unwrap();
    let x2: isize = captures.name("x2").unwrap().as_str().parse().unwrap();
    let y1: isize = captures.name("y1").unwrap().as_str().parse().unwrap();
    let y2: isize = captures.name("y2").unwrap().as_str().parse().unwrap();

    (Coord { x: x1, y: y1 }, Coord { x: x2, y: y2 })
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
