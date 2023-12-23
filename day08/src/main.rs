use itertools::Itertools;
use num::integer::lcm;
use std::collections::HashMap;
type Mapping = HashMap<&'static str, (&'static str, &'static str)>;

fn take_step(mapping: &Mapping, cur: &str, step: &char) -> &'static str {
    let &(left, right) = mapping.get(cur).unwrap();
    match step {
        'L' => left,
        'R' => right,
        _ => unreachable!(),
    }
}

fn get_steps(mapping: &Mapping, steps: &[char], start: &str) -> usize {
    let mut steps_taken = 0;
    let mut cur = start;
    for step in steps.iter().cycle() {
        cur = take_step(mapping, cur, step);
        steps_taken += 1;
        if cur.ends_with('Z') {
            return steps_taken;
        }
    }
    steps_taken
}

fn main() {
    let mut input = include_str!("../input.txt").lines();
    let steps = input.next().unwrap().chars().collect_vec();
    let mapping: Mapping = input.skip(1).fold(HashMap::new(), |mut mapping, line| {
        let (from, rhs) = line.split_once(" = ").unwrap();
        let to = rhs
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split_once(", ")
            .unwrap();
        mapping.insert(from, to);
        mapping
    });

    let p1 = get_steps(&mapping, &steps, "AAA");
    let p2 = &mapping
        .keys()
        .filter(|key| key.ends_with('A'))
        .map(|&start| get_steps(&mapping, &steps, start))
        .fold(1, lcm);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
