use std::collections::HashSet;

type Point = (usize, usize);
type Fold = (char, usize);

pub fn part1(input: &Vec<&str>) -> usize {
    let (mut map, folds) = parse_input(input);

    map = do_fold(&map, folds.first().unwrap());

    map.len()
}

pub fn part2(input: &Vec<&str>) {
    let (mut map, folds) = parse_input(input);

    for fold in folds {
        map = do_fold(&map, &fold);
    }

    for i in 0..6 {
        for j in 0..40 {
            if map.contains(&(j, i)) {
                print!("█");
            } else {
                print!(" ");
            }
        }

        println!();
    }
}

fn parse_input(input: &Vec<&str>) -> (HashSet<Point>, Vec<Fold>) {
    let mut map: HashSet<Point> = HashSet::new();
    let mut folds: Vec<Fold> = Vec::new();
    let mut points_read = false;

    for line in input {
        if line.starts_with("fold") {
            points_read = true;
        }

        if !points_read {
            let split: Vec<usize> = line.split(",").map(|item| item.parse().unwrap()).collect();
            map.insert((split[0], split[1]));
        } else {
            let split: Vec<&str> = line.split("=").collect();

            folds.push((split[0].chars().last().unwrap(), split[1].parse().unwrap()));
        }
    }

    (map, folds)
}

fn do_fold(map: &HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    let mut new_map: HashSet<Point> = HashSet::new();

    for point in map.iter() {
        let new_point = if 'x' == fold.0 && point.0 > fold.1 {
            ((fold.1 - (point.0 - fold.1)), point.1)
        } else if 'y' == fold.0 && point.1 > fold.1 {
            (point.0, (fold.1 - (point.1 - fold.1)))
        } else {
            point.clone()
        };

        new_map.insert(new_point);
    }

    new_map
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 17)
    }
}
