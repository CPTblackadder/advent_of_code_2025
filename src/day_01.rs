use std::u32;

use crate::TaskCompleter;

pub struct Task1;

impl TaskCompleter for Task1 {
    fn do_task_1(&self) -> String {
        let lines = include_str!("../input/day_01/input").lines().map(|x| {
            let d = x.chars().next().unwrap();
            let y = x[1..].parse::<i32>().unwrap();
            (d, y)
        });
        let mut dial = 50;
        let mut count = 0;
        for (direction, amount) in lines {
            if direction == 'L' {
                dial -= amount;
            } else {
                dial += amount
            }
            if dial % 100 == 0 {
                count += 1;
            }
        }
        count.to_string()
    }

    fn do_task_2(&self) -> String {
        let lines = include_str!("../input/day_01/input").lines().map(|x| {
            let d = x.chars().next().unwrap();
            let y = x[1..].parse::<i32>().unwrap();
            (d, y)
        });
        let mut dial = 50;
        let mut count = 0;
        for (direction, amount) in lines {
            
        }
        count.to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("1139".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        None
    }
}
