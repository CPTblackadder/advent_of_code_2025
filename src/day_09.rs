use std::i64;

use itertools::Itertools;

use crate::{TaskCompleter, grid::Coord};

pub struct Task9;

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
        let max_x = coords.iter().map(|x| x.x()).max().unwrap();
        let max_y = coords.iter().map(|x| x.y()).max().unwrap();
        let highest_possible_top_left = Coord::new(0, max_y);
        let highest_possible_bottom_right = Coord::new(max_x, 0);
        let mut top_left = highest_possible_bottom_right;
        let mut bottom_right = highest_possible_top_left;
        let highest_possible_bottom_left = Coord::new(0, 0);
        let highest_possible_top_right = Coord::new(max_x, max_y);
        let mut bottom_left = highest_possible_bottom_right;
        let mut top_right = highest_possible_top_left;

        for c in coords {
            if (highest_possible_top_left - c).squared_len()
                < (highest_possible_top_left - top_left).squared_len()
            {
                top_left = c;
            } else if (highest_possible_bottom_right - c).squared_len()
                < (highest_possible_bottom_right - bottom_right).squared_len()
            {
                bottom_right = c;
            } else if (highest_possible_bottom_left - c).squared_len()
                < (highest_possible_bottom_left - bottom_left).squared_len()
            {
                bottom_left = c;
            } else if (highest_possible_top_right - c).squared_len()
                < (highest_possible_top_right - top_right).squared_len()
            {
                top_right = c;
            }
        }
        dbg!(&bottom_left);
        dbg!(&bottom_right);
        dbg!(&top_left);
        dbg!(&top_right);

        ((bottom_right.x() + 1 - top_left.x()) * (top_left.y() + 1 - bottom_right.y()))
            .max((top_right.x() + 1 - bottom_left.x()) * (top_right.y() + 1 - bottom_left.y()))
            .to_string()
    }

    fn do_task_2(&self) -> String {
        "".to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        None
    }

    fn task_2_result(&self) -> Option<String> {
        None
    }
}
