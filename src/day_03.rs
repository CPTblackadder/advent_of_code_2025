use itertools::Itertools;

use crate::TaskCompleter;

pub struct Task3;

impl TaskCompleter for Task3 {
    fn do_task_1(&self) -> String {
        include_str!("../input/day_03/input")
            .lines()
            .map(|x| {
                let chars = x.chars().collect_vec();
                let mut i = 1;
                let mut largest_first = 0;
                while i < chars.len() - 1 {
                    if chars[i] > chars[largest_first] {
                        largest_first = i;
                    }
                    i += 1;
                }
                i = largest_first + 2;
                let mut largest_second = i - 1;
                while i < chars.len() {
                    if chars[i] > chars[largest_second] {
                        largest_second = i;
                    }
                    i += 1;
                }
                format!("{}{}", chars[largest_first], chars[largest_second])
                    .parse::<i32>()
                    .unwrap()
            })
            .sum::<i32>()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        include_str!("../input/day_03/input")
            .lines()
            .map(|x| {
                let chars = x.chars().collect_vec();
                let mut digits = [0; 12];
                for digit in 0..12 {
                    let mut i = if digit == 0 { 0 } else { digits[digit - 1] + 1 };
                    let mut largest = i;
                    while i < chars.len() - 11 + digit {
                        if chars[i] > chars[largest] {
                            largest = i;
                        }
                        i += 1;
                    }
                    digits[digit] = largest;
                }
                digits.iter().fold(0, |acc, x| {
                    (10 * acc) + i64::from(chars[*x].to_digit(10).unwrap())
                })
            })
            .sum::<i64>()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some(17330.to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        Some(171518260283767i64.to_string())
    }
}
