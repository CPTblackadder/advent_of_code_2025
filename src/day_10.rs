use std::collections::VecDeque;

use crate::TaskCompleter;
use itertools::Itertools;

pub struct Task10;

struct Machine {
    desired_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl Machine {
    fn from_string(str: &str) -> Self {
        let mut split: std::str::Split<'_, &str> = str.split(" ");
        let lights = split
            .next()
            .unwrap()
            .trim_matches(|c| c == '[' || c == ']')
            .chars()
            .map(|x| x == '#')
            .collect_vec();
        let mut split = split.collect_vec();
        let joltage = split
            .pop()
            .unwrap()
            .trim_matches(|c| c == '{' || c == '}')
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect_vec();
        let buttons = split
            .iter()
            .map(|x| {
                x.trim_matches(|c| c == '(' || c == ')')
                    .split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect_vec()
            })
            .collect_vec();
        Self {
            desired_lights: lights,
            buttons,
            joltage,
        }
    }

    fn min_button_presses_for_desired_lights(&self) -> i64 {
        let mut queue = VecDeque::new();
        queue.push_back((vec![false; self.desired_lights.len()], 0));

        while !queue.is_empty() {
            let (state, presses) = queue.pop_front().unwrap();
            if state == self.desired_lights {
                return presses;
            }
            for button in &self.buttons {
                let new_state = press(&state, button);
                queue.push_back((new_state, presses + 1));
            }
        }
        todo!()
    }
}

fn press(state: &Vec<bool>, press: &Vec<usize>) -> Vec<bool> {
    let mut new_state = state.clone();
    for i in press {
        new_state[*i] = !new_state[*i];
    }
    new_state
}

impl TaskCompleter for Task10 {
    fn do_task_1(&self) -> String {
        include_str!("../input/day_10/input")
            .lines()
            .map(|x| Machine::from_string(x).min_button_presses_for_desired_lights())
            .sum::<i64>()
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
