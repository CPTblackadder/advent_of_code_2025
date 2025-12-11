use std::{
    collections::{HashMap, HashSet, VecDeque},
    path::Path,
    rc::Rc,
};

use itertools::{Itertools, enumerate};

use crate::TaskCompleter;

pub struct Task11;

fn get_paths(pos: &str, conns: &HashMap<&str, Vec<&str>>) -> i64 {
    if pos == "out" {
        1
    } else {
        conns[pos].iter().map(|x| get_paths(x, conns)).sum()
    }
}

fn get_no_paths_to_from_each(
    pos: usize,
    conns: &Vec<Vec<usize>>,
    reversed_conns: &Vec<Vec<usize>>,
    name_map: &HashMap<&str, usize>,
) -> Vec<i64> {
    let mut paths_to = vec![0; reversed_conns.len()];
    let mut border = HashSet::from([pos]);
    let mut seen: HashSet<usize> = HashSet::from_iter(conns[pos].iter().map(|x| *x));
    loop {
        let look_at = border
            .iter()
            .find(|x| conns[**x].iter().all(|y| seen.contains(y)))
            .map(|x| *x);
        if let Some(look_at) = look_at {
            seen.insert(look_at);
            border.remove(&look_at);
            paths_to[look_at] = if conns[look_at].is_empty() {
                1
            } else {
                conns[look_at].iter().map(|x| paths_to[*x]).sum()
            };

            border.extend(reversed_conns[look_at].iter().filter(|x| !seen.contains(x)));
            look_at
        } else {
            if border.is_empty() {
                return paths_to;
            } else {
                dbg!(border.iter().map(|x| get_name(*x, name_map)).collect_vec());
                panic!("Dunno");
            }
        };
    }
}

fn get_unreachables(from: usize, conns: &Vec<Vec<usize>>) -> HashSet<usize> {
    let mut border = vec![from];
    let mut seen: HashSet<usize> = HashSet::from_iter(0..conns.len());
    while let Some(x) = border.pop() {
        seen.remove(&x);
        border.extend(conns[x].iter().filter(|y| seen.contains(y)));
    }
    seen
}

fn get_no_paths_to_from_each_1(
    pos: usize,
    conns: &Vec<Vec<usize>>,
    reversed_conns: &Vec<Vec<usize>>,
    name_map: &HashMap<&str, usize>,
) -> Vec<i64> {
    let mut paths_to = vec![0; reversed_conns.len()];
    let mut border = HashSet::from([pos]);
    let mut seen: HashSet<usize> = get_unreachables(pos, reversed_conns);
    loop {
        let look_at = border
            .iter()
            .find_or_first(|x| conns[**x].iter().all(|y| seen.contains(y)))
            .map(|x| *x);
        if let Some(look_at) = look_at {
            if conns[look_at].iter().all(|y| seen.contains(y)) {
                seen.insert(look_at);
            }
            border.remove(&look_at);
            paths_to[look_at] = if look_at == pos {
                1
            } else {
                conns[look_at].iter().map(|x| paths_to[*x]).sum()
            };
            border.extend(reversed_conns[look_at].iter().filter(|x| !seen.contains(x)));
            look_at
        } else {
            if border.is_empty() {
                return paths_to;
            } else {
                dbg!(border.iter().map(|x| get_name(*x, name_map)).collect_vec());
                panic!("Dunno");
            }
        };
    }
}

fn get_name<'a>(i: usize, name_map: &'a HashMap<&'a str, usize>) -> &'a str {
    name_map.iter().find(|(_, x)| **x == i).unwrap().0
}

impl TaskCompleter for Task11 {
    fn do_task_1(&self) -> String {
        let map: HashMap<&str, Vec<&str>> = HashMap::from_iter(
            include_str!("../input/day_11/input")
                .lines()
                .map(|s| (&s[0..3], s[5..].split(" ").collect_vec())),
        );

        get_paths("you", &map).to_string()
    }

    fn do_task_2(&self) -> String {
        let map = include_str!("../input/day_11/input")
            .lines()
            .map(|s| (&s[0..3], s[5..].split(" ").collect_vec()))
            .enumerate()
            .collect_vec();
        let name_map: HashMap<&str, usize> = HashMap::from_iter(
            map.iter()
                .map(|(i, (name, _))| (*name, *i))
                .chain([("out", map.len())]),
        );
        let mut connections: Vec<Vec<usize>> = map
            .iter()
            .map(|(i, (name, conns))| {
                assert_eq!(*i, name_map[name]);
                conns
                    .iter()
                    .filter_map(|x| name_map.get(x).map(|x| *x))
                    .collect_vec()
            })
            .collect_vec();
        connections.push(vec![]);
        let mut reversed_connections: Vec<Vec<usize>> = vec![vec![]; connections.len()];
        for (i, conns) in enumerate(connections.iter()) {
            for j in conns {
                reversed_connections[*j].push(i);
            }
        }
        println!("start compute out");
        let paths_to_out = get_no_paths_to_from_each_1(
            name_map["out"],
            &connections,
            &reversed_connections,
            &name_map,
        );
        assert_eq!(self.do_task_1(), paths_to_out[name_map["you"]].to_string());
        println!("start compute dac");

        let paths_to_dac = get_no_paths_to_from_each_1(
            name_map["dac"],
            &connections,
            &reversed_connections,
            &name_map,
        );
        println!("start compute fft");

        let paths_to_fft = get_no_paths_to_from_each_1(
            name_map["fft"],
            &connections,
            &reversed_connections,
            &name_map,
        );

        ((paths_to_dac[name_map["svr"]]
            * paths_to_fft[name_map["dac"]]
            * paths_to_out[name_map["fft"]])
            + (paths_to_fft[name_map["svr"]]
                * paths_to_dac[name_map["fft"]]
                * paths_to_out[name_map["dac"]]))
            .to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        None
    }

    fn task_2_result(&self) -> Option<String> {
        None
    }
}
