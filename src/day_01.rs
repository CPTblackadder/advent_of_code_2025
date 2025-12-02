use std::u32;

use crate::TaskCompleter;

pub struct Task1;

struct Dial {
    position: i32,
}

impl Dial {
    fn rotate_left(&mut self, amount: i32) -> i32 {
        let full_rotations = amount / 100;
        let remainder = amount % 100;
        let start = self.position;
        self.position = self.position - remainder;
        let extra = if remainder > 0 {
            if self.position == 0 {
                1
            } else if self.position < 0 {
                self.position += 100;
                if start != 0 { 1 } else { 0 }
            } else {
                0
            }
        } else {
            0
        };
        // println!(
        //     "-{amount} turn {} -> {}, adding {}",
        //     start,
        //     self.position,
        //     full_rotations + extra
        // );
        assert!(self.position >= 0);
        assert!(self.position < 100);
        full_rotations + extra
    }

    fn rotate_right(&mut self, amount: i32) -> i32 {
        let full_rotations = amount / 100;
        let remainder = amount % 100;
        let start = self.position;
        self.position = self.position + remainder;
        let extra = if remainder > 0 {
            if self.position == 0 {
                1
            } else if self.position >= 100 {
                self.position -= 100;
                1
            } else {
                0
            }
        } else {
            0
        };
        // println!(
        //     "+{amount} turn {} -> {}, adding {}",
        //     start,
        //     self.position,
        //     full_rotations + extra
        // );
        assert!(self.position >= 0);
        assert!(self.position < 100);
        full_rotations + extra
    }
}

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
        let mut dial = Dial { position: 50 };
        let mut count = 0;
        for (direction, amount) in lines {
            count += if direction == 'L' {
                dial.rotate_left(amount)
            } else {
                dial.rotate_right(amount)
            }
        }
        count.to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("1139".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("6684".to_string())
    }
}
