pub type Coord = (usize, usize);

pub struct Grid<T> {
    pub size: (usize, usize),
    cells: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(size: (usize, usize), cells: Vec<T>) -> Self {
        assert_eq!(size.0 * size.1, cells.len());

        Grid { size, cells }
    }

    pub fn cell(&self, coord: Coord) -> &T {
        &self.cells[self.index(coord)]
    }

    pub fn neighbors(&self, coord: Coord) -> Vec<(Coord, &T)> {
        let mut neighbors = Vec::with_capacity(4);

        for neighbor_coord in self.neighbor_coord_iter(coord) {
            neighbors.push((neighbor_coord, self.cell(neighbor_coord)));
        }

        neighbors
    }

    fn neighbor_coord_iter(&self, coord: Coord) -> NeighborIterator<T> {
        assert!(coord.0 < self.size.0);
        assert!(coord.1 < self.size.1);

        NeighborIterator::new(&self, coord)
    }

    fn index(&self, coord: Coord) -> usize {
        assert!(coord.0 < self.size.0);
        assert!(coord.1 < self.size.1);

        coord.0 + coord.1 * self.size.0
    }
}

struct NeighborIterator<'a, T> {
    grid: &'a Grid<T>,
    coord: Coord,
    visited: Vec<usize>,
}

impl<'a, T> NeighborIterator<'a, T> {
    pub fn new(grid: &'a Grid<T>, coord: Coord) -> Self {
        NeighborIterator {
            grid,
            coord,
            visited: vec![],
        }
    }
}

impl<'a, T> Iterator for NeighborIterator<'a, T> {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.visited.contains(&0) && self.coord.0 != 0 {
            self.visited.push(0);
            Some((self.coord.0 - 1, self.coord.1))
        } else if !self.visited.contains(&1) && self.coord.1 != 0 {
            self.visited.push(1);
            Some((self.coord.0, self.coord.1 - 1))
        } else if !self.visited.contains(&2) && self.coord.0 + 1 != self.grid.size.0 {
            self.visited.push(2);
            Some((self.coord.0 + 1, self.coord.1))
        } else if !self.visited.contains(&3) && self.coord.1 + 1 != self.grid.size.1 {
            self.visited.push(3);
            Some((self.coord.0, self.coord.1 + 1))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_grid() -> Grid<char> {
        Grid::new(
            (4, 5),
            [
                'a', 'b', 'c', 'd', //
                'e', 'f', 'g', 'h', //
                'i', 'j', 'k', 'l', //
                'm', 'n', 'o', 'p', //
                'q', 'r', 's', 't', //
            ]
            .to_vec(),
        )
    }

    #[test]
    fn test_get_a_cell() {
        let grid = create_grid();
        assert_eq!(*grid.cell((0, 0)), 'a');
        assert_eq!(*grid.cell((2, 3)), 'o');
        assert_eq!(*grid.cell((3, 4)), 't');
    }

    #[test]
    fn test_get_the_cell_neighbors() {
        let grid = create_grid();

        assert_eq!(
            grid.neighbors((0, 0)),
            [((1, 0), &'b'), ((0, 1), &'e')].to_vec()
        );
        assert_eq!(
            grid.neighbors((2, 0)),
            [((1, 0), &'b'), ((3, 0), &'d'), ((2, 1), &'g')].to_vec()
        );
        assert_eq!(
            grid.neighbors((1, 2)),
            [
                ((0, 2), &'i'),
                ((1, 1), &'f'),
                ((2, 2), &'k'),
                ((1, 3), &'n')
            ]
            .to_vec()
        );
        assert_eq!(
            grid.neighbors((3, 4)),
            [((2, 4), &'s'), ((3, 3), &'p')].to_vec()
        );
    }
}
