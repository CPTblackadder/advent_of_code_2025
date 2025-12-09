use std::{collections::HashMap};

use itertools::Itertools;

use crate::TaskCompleter;

pub struct Task8;

impl TaskCompleter for Task8 {
    fn do_task_1(&self) -> String {
        let coords = include_str!("../input/day_08/input").lines().map(|x| {
            let mut numbers = x.split(",");
            let x = numbers.next().unwrap().parse::<i64>().unwrap();
            let y = numbers.next().unwrap().parse::<i64>().unwrap();
            let z = numbers.next().unwrap().parse::<i64>().unwrap();
            (x, y, z)
        });
        let pairs = coords
            .clone()
            .cartesian_product(coords)
            .sorted_by_key(|(x, y)| (x.0 - y.0).pow(2) + (x.1 - y.1).pow(2) + (x.2 - y.2).pow(2))
            .filter(|(x, y)| x < y)
            .take(1000);

        let mut h = HashMap::new();
        let mut circuits = 0;

        for (x, y) in pairs {
            if h.contains_key(&x) {
                if h.contains_key(&y) {
                    let combined = h[&x];
                    let override_val = h[&y];
                    if combined != override_val {
                        // println!(
                        //     "Combining {x:?} and {y:?} from {combined} and {override_val} into {combined}"
                        // );
                        for v in h.values_mut() {
                            if *v == override_val {
                                *v = combined;
                            }
                        }
                    } else {
                        // println!("Skipping {x:?} and {y:?} as they are in the same circuit");
                    }
                } else {
                    // println!("Adding {y:?} to {x:?} as {}", h[&x]);
                    h.insert(y, h[&x]);
                }
            } else if h.contains_key(&y) {
                // println!("Adding {x:?} to {y:?} as {}", h[&y]);
                h.insert(x, h[&y]);
            } else {
                // println!("Adding {x:?} and {y:?} as {circuits}");
                h.insert(x, circuits);
                h.insert(y, circuits);
                circuits += 1;
            }
        }
        let mut count = vec![0; circuits];
        for v in h.values() {
            count[*v] += 1;
        }

        count.sort();
        count.iter().rev().take(3).fold(1, |x, y| x * y).to_string()
    }

    fn do_task_2(&self) -> String {
        let coords = include_str!("../input/day_08/input")
            .lines()
            .map(|x| {
                let mut numbers = x.split(",");
                let x = numbers.next().unwrap().parse::<i64>().unwrap();
                let y = numbers.next().unwrap().parse::<i64>().unwrap();
                let z = numbers.next().unwrap().parse::<i64>().unwrap();
                (x, y, z)
            })
            .zip(0..);
        let pairs = coords
            .clone()
            .cartesian_product(coords.clone())
            .sorted_by_key(|((x, _), (y, _))| {
                (x.0 - y.0).pow(2) + (x.1 - y.1).pow(2) + (x.2 - y.2).pow(2)
            })
            .filter(|(x, y)| x < y);

        let mut h = vec![None; coords.count()];
        let mut circuits = 0;

        for ((x, x_i), (y, y_i)) in pairs {
            if !h[x_i].is_none() {
                if !h[y_i].is_none() {
                    let combined = h[x_i];
                    let override_val = h[y_i];
                    if combined != override_val {
                        // println!(
                        //     "Combining {x:?} and {y:?} from {combined} and {override_val} into {combined}"
                        // );
                        for v in h.iter_mut() {
                            if *v == override_val {
                                *v = combined;
                            }
                        }
                    } else {
                        // println!("Skipping {x:?} and {y:?} as they are in the same circuit");
                    }
                } else {
                    // println!("Adding {y:?} to {x:?} as {}", h[&x]);
                    h[y_i] = h[x_i];
                }
            } else if !h[y_i].is_none() {
                // println!("Adding {x:?} to {y:?} as {}", h[&y]);
                h[x_i] = h[y_i];
            } else {
                // println!("Adding {x:?} and {y:?} as {circuits}");
                h[x_i] = Some(circuits);
                h[y_i] = Some(circuits);
                circuits += 1;
            }
            if h.iter().all_equal() {
                return (x.0 * y.0).to_string();
            }
        }

        "ERROR".to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        None
    }

    fn task_2_result(&self) -> Option<String> {
        None
    }
}
