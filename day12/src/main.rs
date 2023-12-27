use cached::proc_macro::cached;
use itertools::Itertools;
use std::iter::repeat;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Record {
    conditions: String,
    required: Vec<usize>,
}

#[cached]
fn possible_arrangements(record: Record) -> usize {
    if record.required.is_empty() {
        if record.conditions.is_empty() || record.conditions.chars().all(|c| c != '#') {
            return 1;
        }
        return 0;
    }
    if record.conditions.len()
        < (record.required.iter().sum::<usize>() + (record.required.len() - 1))
    {
        return 0;
    }

    match record.conditions.chars().next() {
        Some('.') => {
            return possible_arrangements(Record {
                conditions: String::from(&record.conditions[1..]),
                required: record.required.clone(),
            });
        }
        Some('#') => {
            let (&required, rest) = record.required.split_first().unwrap();
            if record.conditions[..required].chars().all(|c| c != '.')
                && (record.conditions.len() == required
                    || &record.conditions[required..=required] != "#")
            {
                if required + 1 > record.conditions.len() {
                    return 1;
                }
                return possible_arrangements(Record {
                    conditions: String::from(&record.conditions[required + 1..]),
                    required: Vec::from(rest),
                });
            }
            return 0;
        }
        Some('?') => {
            return possible_arrangements(Record {
                conditions: record.conditions.clone().replacen('?', ".", 1),
                required: record.required.clone(),
            }) + possible_arrangements(Record {
                conditions: record.conditions.clone().replacen('?', "#", 1),
                required: record.required.clone(),
            });
        }
        _ => unreachable!(),
    }
}
fn main() {
    let records = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let (conditions, required_str) = line.split_once(' ').unwrap();
            let required = required_str
                .split(',')
                .map(|digit| digit.parse().unwrap())
                .collect_vec();
            Record {
                conditions: conditions.to_owned(),
                required,
            }
        })
        .collect_vec();

    let p1 = records
        .iter()
        .cloned()
        .map(possible_arrangements)
        .sum::<usize>();

    let p2 = records
        .iter()
        .map(|record| Record {
            conditions: repeat(record.conditions.clone()).take(5).join("?"),
            required: record.required.repeat(5),
        })
        .map(possible_arrangements)
        .sum::<usize>();

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
