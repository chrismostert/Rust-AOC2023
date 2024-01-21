use num::integer::lcm;
use std::{
    collections::{HashMap, VecDeque},
    iter,
};

#[derive(Debug, Clone)]
struct Module {
    name: String,
    destinations: Vec<String>,
    module_type: ModuleType,
}
impl Module {
    fn send_pulse(&mut self, from: String, high: bool) -> Option<Vec<(String, bool)>> {
        if let Some(pulse) = match self.module_type {
            ModuleType::Flipflop(ref mut state) => {
                if high {
                    return None;
                }
                *state = !*state;
                Some(*state)
            }
            ModuleType::Conjunction(ref mut inputs) => {
                *inputs.get_mut(&from).unwrap() = high;
                Some(!inputs.values().all(|&val| val))
            }
            ModuleType::Broadcaster => Some(high),
        } {
            return Some(
                self.destinations
                    .iter()
                    .map(|dest| (dest.to_owned(), pulse))
                    .collect::<Vec<_>>(),
            );
        }
        None
    }
}

#[derive(Debug, Clone)]
enum ModuleType {
    Broadcaster,
    Flipflop(bool),
    Conjunction(HashMap<String, bool>),
}

fn parse_input(input: &str) -> HashMap<String, Module> {
    let mut modules = input
        .lines()
        .map(|line| {
            let (kind, dests_str) = line.split_once(" -> ").unwrap();
            let destinations = dests_str
                .split(", ")
                .map(|str| str.to_string())
                .collect::<Vec<_>>();
            let name = kind
                .chars()
                .filter(|&char| char != '%' && char != '&')
                .collect::<String>();
            let module = {
                Module {
                    name,
                    destinations,
                    module_type: match &kind[0..1] {
                        "%" => ModuleType::Flipflop(false),
                        "&" => ModuleType::Conjunction(HashMap::new()),
                        _ => ModuleType::Broadcaster,
                    },
                }
            };
            (module.name.clone(), module)
        })
        .collect::<HashMap<_, _>>();

    // Set conjunction hashmaps
    for (dest, source) in modules
        .iter()
        .flat_map(|(source, module)| {
            module
                .destinations
                .iter()
                .map(|dest| (dest.to_owned(), source.to_owned()))
        })
        .collect::<Vec<_>>()
    {
        if let Some(module) = modules.get_mut(&dest) {
            if let ModuleType::Conjunction(ref mut inputs) = module.module_type {
                inputs.insert(source, false);
            }
        }
    }

    modules
}

fn push_button(
    mut modules: HashMap<String, Module>,
    times: usize,
    destination_to_watch: String,
    sources_to_watch: Vec<String>,
) -> (usize, usize) {
    let mut n_low = 0;
    let mut n_high = 0;
    let mut times_pushed = 0;
    let mut low_high = None;
    let mut cycle_lengths = sources_to_watch
        .iter()
        .zip(iter::repeat(None))
        .collect::<HashMap<&String, Option<usize>>>();

    let mut queue = VecDeque::new();

    loop {
        times_pushed += 1;
        queue.push_back((String::from("button"), String::from("broadcaster"), false));

        while let Some((source, dest, high)) = queue.pop_front() {
            if high {
                n_high += 1;
            } else {
                n_low += 1;
            }

            if let Some(dest_module) = modules.get_mut(&dest) {
                if let Some(new_pulses) = dest_module.send_pulse(source, high) {
                    for (dest, high) in new_pulses {
                        // Track shortest cycle lengths
                        if sources_to_watch.contains(&dest_module.name)
                            && destination_to_watch == dest
                            && high
                            && cycle_lengths.get(&dest_module.name).unwrap().is_none()
                        {
                            *cycle_lengths.get_mut(&dest_module.name).unwrap() = Some(times_pushed);
                        }
                        // Add new signal to queue
                        queue.push_back((dest_module.name.clone(), dest, high));
                    }
                }
            }
        }
        // Save the low*high count but keep pushing for p2
        if times_pushed == times {
            low_high = Some(n_low * n_high);
        }
        // We have all the cycle lengths
        if let Some(low_high) = low_high {
            if cycle_lengths.values().all(|val| val.is_some()) {
                return (
                    low_high,
                    cycle_lengths
                        .values()
                        .flatten()
                        .fold(1, |acc, &cycle_length| lcm(acc, cycle_length)),
                );
            }
        }
    }
}

fn main() {
    let modules = parse_input(include_str!("../input.txt"));

    // For part two we need a low pulse to rx, given the input this requires the following conjunction
    // node to high pulses from all *its* inputs.
    let destination_to_watch = modules
        .iter()
        .find(|(_, module)| module.destinations.contains(&String::from("rx")))
        .unwrap()
        .0
        .to_owned();
    let sources_to_watch = modules
        .iter()
        .filter(|(_, module)| module.destinations.contains(&destination_to_watch))
        .map(|(name, _)| name.to_owned())
        .collect::<Vec<_>>();

    let (p1, p2) = push_button(modules, 1_000, destination_to_watch, sources_to_watch);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
