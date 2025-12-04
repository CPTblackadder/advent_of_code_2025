use itertools::Itertools;

use crate::{TaskCompleter, grid::Grid};

pub struct Task4;

impl TaskCompleter for Task4 {
    fn do_task_1(&self) -> String {
        let grid = Grid::from_string(include_str!("../input/day_04/input"), false);
        grid.iter()
            .filter(|x| {
                *x.1 == '@'
                    && x.0
                        .get_bounded_neighbours(&grid, true)
                        .iter()
                        .filter(|x| grid[**x] == '@')
                        .count()
                        < 4
            })
            .count()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let mut grid = Grid::from_string(include_str!("../input/day_04/input"), false);
        let mut removed = 0;
        loop {
            let to_remove = grid
                .iter()
                .filter_map(|x| {
                    if *x.1 == '@'
                        && x.0
                            .get_bounded_neighbours(&grid, true)
                            .iter()
                            .filter(|x| grid[**x] == '@')
                            .count()
                            < 4
                    {
                        Some(x.0)
                    } else {
                        None
                    }
                })
                .collect_vec();
            if to_remove.is_empty() {
                break;
            }
            removed += to_remove.len();
            for coord in to_remove {
                grid[coord] = '.'
            }
        }
        removed.to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some(1351.to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        Some(8345.to_string())
    }
}
