use std::{fmt::Display, i64};

use itertools::Itertools;

use crate::{TaskCompleter, grid::Coord, sparsegrid::SparseGrid};

pub struct Task9;

#[derive(Clone, Default, PartialEq, Eq, Debug)]
enum GridSpace {
    #[default]
    Inside,
    Border,
    Outside,
}

impl Display for GridSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            GridSpace::Inside => "O",
            GridSpace::Border => "X",
            GridSpace::Outside => ".",
        };
        f.write_str(&str)
    }
}

impl TaskCompleter for Task9 {
    fn do_task_1(&self) -> String {
        let coords = include_str!("../input/day_09/input")
            .lines()
            .map(|x| {
                let mut s = x.split(",");
                let x = s.next().unwrap().parse::<i64>().unwrap();
                let y = s.next().unwrap().parse::<i64>().unwrap();
                Coord::new(x, y)
            })
            .collect_vec();
        coords
            .iter()
            .cartesian_product(coords.iter())
            .map(|(x, y)| (x.x().abs_diff(y.x()) + 1) * (x.y().abs_diff(y.y()) + 1))
            .max()
            .unwrap()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let coords = include_str!("../input/day_09/input")
            .lines()
            .map(|x| {
                let mut s = x.split(",");
                let x = s.next().unwrap().parse::<i64>().unwrap();
                let y = s.next().unwrap().parse::<i64>().unwrap();
                Coord::new(x, y)
            })
            .collect_vec();
        let max_x = coords.iter().map(|x| x.x()).max().unwrap();
        let max_y = coords.iter().map(|x| x.y()).max().unwrap();

        let mut grid: SparseGrid<GridSpace> =
            SparseGrid::default_with_size((max_x as usize + 2, max_y as usize + 2), (100, 100));
        for (x, y) in coords
            .iter()
            .zip(coords.iter().skip(1))
            .chain([(coords.last().unwrap(), &coords[0])])
        {
            if x.x() == y.x() {
                let higher = x.y().max(y.y());
                let lower = x.y().min(y.y());
                for i in lower..higher + 1 {
                    grid.set(Coord::new(x.x(), i), GridSpace::Border);
                }
            } else {
                assert_eq!(x.y(), y.y());
                let higher = x.x().max(y.x());
                let lower = x.x().min(y.x());
                for i in lower..higher + 1 {
                    grid.set(Coord::new(i, x.y()), GridSpace::Border);
                }
            }
        }

        grid.do_flood_fill(Coord::new(0, 0), GridSpace::Outside, GridSpace::Inside);

        let (x, y) = coords
            .iter()
            .cartesian_product(coords.iter())
            .sorted_by_key(|(x, y)| (x.x().abs_diff(y.x()) + 1) * (x.y().abs_diff(y.y()) + 1))
            .rev()
            .filter(|(x, y)| {
                let mut xs = [x.x(), y.x()];
                let mut ys = [x.y(), y.y()];
                xs.sort();
                ys.sort();
                let [x_0, x_1] = xs;
                let [y_0, y_1] = ys;
                (x_0..x_1).all(|x| {
                    grid[Coord::new(x, y_0)] != GridSpace::Outside
                        && grid[Coord::new(x, y_1)] != GridSpace::Outside
                }) && (y_0..y_1).all(|y| {
                    grid[Coord::new(x_0, y)] != GridSpace::Outside
                        && grid[Coord::new(x_1, y)] != GridSpace::Outside
                })
            })
            .next()
            .unwrap();
        ((x.x().abs_diff(y.x()) + 1) * (x.y().abs_diff(y.y()) + 1)).to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some(4760959496i64.to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        None
    }
}
