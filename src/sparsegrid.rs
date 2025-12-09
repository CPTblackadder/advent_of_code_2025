use std::{
    fmt::{Debug, Display},
    ops::Index,
};

use itertools::Either;

use crate::grid::{Coord, Grid};

pub struct SparseGrid<T: Clone> {
    size: (usize, usize),
    subsize: (usize, usize),
    grid: Grid<Either<T, Box<Grid<T>>>>,
}

impl<T: Display + Clone> Display for SparseGrid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = "\n".to_owned();
        for y in (0..self.size.1).rev() {
            let mut l = "".to_owned();

            for x in 0..self.size.0 {
                l += &self[Coord::new(x as i64, y as i64)].to_string();
            }
            str += &l;
            str += "\n";
        }
        f.write_str(&str)
    }
}

impl<T: Clone> Index<Coord> for SparseGrid<T> {
    type Output = T;

    fn index(&self, index: Coord) -> &Self::Output {
        let (x, y) = (
            index.x() / self.subsize.0 as i64,
            index.y() / self.subsize.1 as i64,
        );
        let (x_2, y_2) = (
            index.x() % self.subsize.0 as i64,
            index.y() % self.subsize.1 as i64,
        );
        match &self.grid[Coord::new(x, y)] {
            Either::Left(t) => t,
            Either::Right(g) => &g[Coord::new(x_2, y_2)],
        }
    }
}

impl<T: Default + Clone> SparseGrid<T> {
    pub fn default_with_size(size: (usize, usize), subsize: (usize, usize)) -> Self {
        Self {
            size,
            subsize,
            grid: Grid::init_with_size(
                itertools::Either::Left(T::default()),
                (size.0 / subsize.0) + 1,
                (size.1 / subsize.1) + 1,
            ),
        }
    }
}

impl<T: Clone> SparseGrid<T> {
    pub fn init_with_size(size: (usize, usize), subsize: (usize, usize), default: T) -> Self {
        Self {
            size,
            subsize,
            grid: Grid::init_with_size(
                itertools::Either::Left(default),
                (size.0 / subsize.0) + 1,
                (size.1 / subsize.1) + 1,
            ),
        }
    }

    pub fn in_bounds(&self, coord: Coord) -> bool {
        coord.x() >= 0
            && coord.x() < self.size.0 as i64
            && coord.y() >= 0
            && coord.y() < self.size.1 as i64
    }
}

impl<T: Clone + PartialEq + Debug + Display> SparseGrid<T> {
    pub fn do_flood_fill(&mut self, centre: Coord, to: T, unfilled: T) {
        let mut queue = vec![centre];

        while let Some(coord) = queue.pop() {
            if self.in_bounds(coord) && self[coord] == unfilled {
                let (x_1, y_1) = (
                    coord.x() / self.subsize.0 as i64,
                    coord.y() / self.subsize.1 as i64,
                );
                let (x_2, y_2) = (
                    coord.x() % self.subsize.0 as i64,
                    coord.y() % self.subsize.1 as i64,
                );
                let (bottom_left_x, bottom_left_y) =
                    (x_1 * self.subsize.0 as i64, y_1 * self.subsize.1 as i64);
                let (top_right_x, top_right_y) = (
                    (x_1 + 1) * self.subsize.0 as i64,
                    (y_1 + 1) * self.subsize.1 as i64,
                );

                let large_grid_coord = Coord::new(x_1, y_1);
                if let Either::Right(g) = &mut self.grid[large_grid_coord] {
                    g[Coord::new(x_2, y_2)] = to.clone();
                    queue.append(&mut coord.get_neighbours(false));
                } else {
                    let prev_value = self.grid[large_grid_coord].clone().left().unwrap();
                    assert_ne!(prev_value, to.clone());
                    self.grid[Coord::new(x_1, y_1)] = Either::Left(to.clone());
                    queue.extend(
                        (0..self.subsize.0)
                            .map(|i| Coord::new(bottom_left_x + i as i64, bottom_left_y - 1)),
                    );
                    queue.extend(
                        (0..self.subsize.0)
                            .map(|i| Coord::new(bottom_left_x + i as i64, top_right_y)),
                    );
                    queue.extend(
                        (0..self.subsize.1)
                            .map(|i| Coord::new(bottom_left_x - 1, bottom_left_y + i as i64)),
                    );
                    queue.extend(
                        (0..self.subsize.1)
                            .map(|i| Coord::new(top_right_x, bottom_left_y + i as i64)),
                    );
                }
            }
        }
    }

    pub fn set(&mut self, index: Coord, clone: T) {
        let (x, y) = (
            index.x() / self.subsize.0 as i64,
            index.y() / self.subsize.1 as i64,
        );
        let (x_2, y_2) = (
            index.x() % self.subsize.0 as i64,
            index.y() % self.subsize.1 as i64,
        );
        if let Either::Right(g) = &mut self.grid[Coord::new(x, y)] {
            g[Coord::new(x_2, y_2)] = clone
        } else {
            let prev_value = self.grid[Coord::new(x, y)].clone().left().unwrap();
            if prev_value != clone {
                let mut new_grid = Grid::init_with_size(prev_value, self.subsize.0, self.subsize.1);
                new_grid[Coord::new(x_2, y_2)] = clone;
                self.grid[Coord::new(x, y)] = Either::Right(Box::new(new_grid));
            }
        }
    }
}
