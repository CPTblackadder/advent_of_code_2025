use itertools::Itertools;

use crate::TaskCompleter;

pub struct Task5;

impl TaskCompleter for Task5 {
    fn do_task_1(&self) -> String {
        let lines = include_str!("../input/day_05/input").lines().collect_vec();
        let ranges = lines
            .iter()
            .take(187)
            .map(|x| {
                let mut s = x.split("-");
                let first = s.next().unwrap().parse::<i64>().unwrap();
                let second = s.next().unwrap().parse::<i64>().unwrap();
                assert!(first <= second);
                first..(second + 1)
            })
            .collect_vec();

        lines
            .iter()
            .skip(188)
            .map(|x| x.parse::<i64>().unwrap())
            .filter(|x| ranges.iter().any(|r| r.contains(x)))
            .count()
            .to_string()
    }

    fn do_task_2(&self) -> String {
        let ranges = include_str!("../input/day_05/input")
            .lines()
            .take(187)
            .map(|x| {
                let mut s = x.split("-");
                let first = s.next().unwrap().parse::<usize>().unwrap();
                let second = s.next().unwrap().parse::<usize>().unwrap();
                assert!(first <= second);
                first..(second + 1)
            })
            .sorted_by_key(|x| x.start)
            .collect_vec();

        ranges
            .into_iter()
            .fold(vec![], |mut c, i| {
                if c.is_empty() {
                    vec![i]
                } else {
                    let last = c.last_mut().unwrap();
                    if last.end >= i.start {
                        // Merge two ranges
                        if last.end < i.end {
                            last.end = i.end
                        }
                        // Otherwise no need to do anything
                    } else {
                        // Add other range
                        c.push(i);
                    }
                    c
                }
            })
            .iter()
            .map(|x| ExactSizeIterator::len(x))
            .sum::<usize>()
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some(862.to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        Some(357907198933892i64.to_string())
    }
}
