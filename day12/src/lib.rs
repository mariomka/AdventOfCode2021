use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use petgraph::graph::{NodeIndex, UnGraph};

type PathGraph<'a> = UnGraph<Node<'a>, usize>;

pub fn part1(input: &Vec<&str>) -> usize {
    let (graph, nodes) = parse_graph(input);

    get_paths(&graph, nodes["start"], nodes["end"], false).len()
}

pub fn part2(input: &Vec<&str>) -> usize {
    let (graph, nodes) = parse_graph(input);

    get_paths(&graph, nodes["start"], nodes["end"], true).len()
}

fn parse_graph<'a>(input: &Vec<&'a str>) -> (PathGraph<'a>, HashMap<&'a str, NodeIndex>) {
    let mut nodes = HashMap::new();
    let mut graph = PathGraph::new_undirected();

    input.iter().for_each(|line| {
        let mut split = line.split('-');
        let a = split.next().unwrap();
        let b = split.next().unwrap();

        let a = *nodes.entry(a).or_insert_with(|| {
            let node = Node::new(a);
            graph.add_node(node)
        });

        let b = *nodes.entry(b).or_insert_with(|| {
            let node = Node::new(b);
            graph.add_node(node)
        });

        graph.add_edge(a, b, 0);
    });

    (graph, nodes)
}

fn get_paths(
    graph: &PathGraph,
    from: NodeIndex,
    to: NodeIndex,
    with_one_small_twice: bool,
) -> Vec<Vec<NodeIndex>> {
    let mut stack = [(
        graph.neighbors(from),
        [from].to_vec(),
        [from].to_vec(),
        None,
        false,
    )]
    .to_vec();
    let mut paths = Vec::new();

    while let Some((neighbors, path, visited, small_twice, visited_twice)) = stack.last_mut() {
        if let Some(neighbor) = neighbors.next() {
            if !visited.contains(&neighbor)
                || (small_twice.is_some()
                    && neighbor == small_twice.unwrap()
                    && *visited_twice == false)
            {
                let node = graph.node_weight(neighbor).unwrap();
                let small_twice = small_twice.clone();
                let mut visited_twice = visited_twice.clone();

                if visited.contains(&neighbor) {
                    visited_twice = true;
                }

                let mut visited = visited.clone();

                if Kind::SmallCave == node.kind {
                    visited.push(neighbor);
                }

                let mut path = path.clone();
                path.push(neighbor);

                if neighbor == to {
                    paths.push(path);
                } else {
                    stack.push((
                        graph.neighbors(neighbor),
                        path.clone(),
                        visited.clone(),
                        small_twice,
                        visited_twice,
                    ));

                    if with_one_small_twice && Kind::SmallCave == node.kind && small_twice.is_none()
                    {
                        stack.push((
                            graph.neighbors(neighbor),
                            path.clone(),
                            visited.clone(),
                            Some(neighbor),
                            visited_twice,
                        ));
                    }
                }
            }
        } else {
            stack.pop();
        }
    }

    if with_one_small_twice {
        paths.sort_unstable();
        paths.dedup();
    }

    paths
}

#[derive(Debug, PartialEq)]
enum Kind {
    Start,
    End,
    SmallCave,
    LargeCave,
}

#[derive(Debug)]
struct Node<'a> {
    kind: Kind,
    name: &'a str,
}

impl<'a> Node<'a> {
    fn new(name: &'a str) -> Self {
        let kind = match name {
            "start" => Kind::Start,
            "end" => Kind::End,
            _ => {
                if name == name.to_uppercase() {
                    Kind::LargeCave
                } else {
                    Kind::SmallCave
                }
            }
        };

        Node { kind, name }
    }
}

impl<'a> Display for Node<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use helpers::input_lines;

    use super::*;

    fn input<'a>() -> Vec<&'a str> {
        let input = "\
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
        ";
        input_lines(input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input()), 10)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input()), 36)
    }
}
