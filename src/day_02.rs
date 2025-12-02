use std::collections::HashSet;

use itertools::Itertools;

use crate::TaskCompleter;

pub struct Task2;

struct RangeCollection {
    ranges: Vec<(i64, i64)>,
}

impl RangeCollection {
    fn construct<T: Iterator<Item = (i64, i64)>>(ranges: T) -> Self {
        let mut rc = Self { ranges: vec![] };
        for (start, end) in ranges {
            assert!(!rc.is_in(start));
            assert!(!rc.is_in(end));
            rc.ranges.push((start, end));
        }
        rc.ranges.sort();
        rc
    }

    fn is_in(&self, value: i64) -> bool {
        for (start, end) in &self.ranges {
            if value < *start {
                return false;
            } else if value <= *end {
                return true;
            }
        }
        false
    }

    fn max(&self) -> i64 {
        self.ranges.last().unwrap().1
    }
}

impl TaskCompleter for Task2 {
    fn do_task_1(&self) -> String {
        let ranges =
            RangeCollection::construct(include_str!("../input/day_02/input").split(",").map(|x| {
                let mut s = x.split("-");
                let first = s.next().unwrap().trim().parse::<i64>().unwrap();
                let second = s.next().unwrap().trim().parse::<i64>().unwrap();
                (first, second)
            }));

        let max_str = ranges.max().to_string();
        let max_length = max_str.len();
        let max = max_str
            .chars()
            .take(max_length / 2)
            .collect::<String>()
            .parse::<i64>()
            .unwrap();

        (0..max)
            .map(|x| {
                let v = format!("{}{}", x, x).parse::<i64>().unwrap();
                if ranges.is_in(v) { v } else { 0 }
            })
            .sum::<i64>()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let ranges =
            RangeCollection::construct(include_str!("../input/day_02/input").split(",").map(|x| {
                let mut s = x.split("-");
                let first = s.next().unwrap().trim().parse::<i64>().unwrap();
                let second = s.next().unwrap().trim().parse::<i64>().unwrap();
                (first, second)
            }));

        let mut v = 1;
        let mut coll = HashSet::new();
        let mut count = 0;
        loop {
            let mut x = format!("{}{}", v, v).parse::<i64>().unwrap();
            if x > ranges.max() {
                break;
            }
            if !coll.contains(&x) {
                if ranges.is_in(x) {
                    count += x;
                    coll.insert(x);
                }
            }

            while x <= ranges.max() {
                x = format!("{}{}", x, v).parse::<i64>().unwrap();
                if !coll.contains(&x) {
                    if ranges.is_in(x) {
                        count += x;
                        coll.insert(x);
                    }
                }
            }

            v += 1;
        }

        count.to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("21898734247".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("28915664389".to_string())
    }
}
