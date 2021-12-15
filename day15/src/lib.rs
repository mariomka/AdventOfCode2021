use std::collections::HashMap;

use helpers::{Coord, Grid};
use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

pub fn part1(input: &Grid<usize>) -> usize {
    let (nodes, graph) = make_graph(input);
    let start = *nodes.get(&(0, 0)).unwrap();
    let dest = *nodes.get(&(input.size.0 - 1, input.size.1 - 1)).unwrap();

    find_lowest_risk(&graph, start, dest)
}

pub fn part2(input: &Grid<usize>) -> usize {
    let mut cells: Vec<usize> = Vec::new();

    for y in 0..input.size.1 * 5 {
        for x in 0..input.size.0 * 5 {
            let original_risk = *input.get((x % input.size.0, y % input.size.1));

            if x < input.size.0 && y < input.size.1 {
                cells.push(original_risk);
                continue;
            }

            let moves = x / input.size.0 + y / input.size.1;
            let risk = ((original_risk + moves - 1) % 9) + 1;

            cells.push(risk);
        }
    }

    let grid = Grid::new((input.size.0 * 5, input.size.1 * 5), cells);
    let (nodes, graph) = make_graph(&grid);
    let start = *nodes.get(&(0, 0)).unwrap();
    let dest = *nodes.get(&(grid.size.0 - 1, grid.size.1 - 1)).unwrap();

    find_lowest_risk(&graph, start, dest)
}

fn make_graph(input: &Grid<usize>) -> (HashMap<Coord, NodeIndex>, Graph<(), usize>) {
    let mut graph = Graph::new();
    let mut nodes = HashMap::new();

    for (coord, _) in input.iter() {
        nodes.insert(coord, graph.add_node(()));
    }

    for (coord, node_index) in nodes.iter() {
        for (coord, risk) in input.neighbors_iter(*coord, false) {
            graph.add_edge(*node_index, *nodes.get(&coord).unwrap(), *risk);
        }
    }

    (nodes, graph)
}

fn find_lowest_risk(graph: &Graph<(), usize>, start: NodeIndex, dest: NodeIndex) -> usize {
    astar(graph, start, |_dest| dest == _dest, |e| *e.weight(), |_| 0)
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use helpers::{input_grid, Grid};

    use super::*;

    fn input<'a>() -> Grid<usize> {
        let input = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        input_grid(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 40)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 315)
    }
}
