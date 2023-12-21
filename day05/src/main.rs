use itertools::Itertools;
use std::ops::Range;

struct Mapping {
    range: Range<isize>,
    transformation: isize,
}

fn parse_input(input: &str) -> (Vec<isize>, Vec<Vec<Mapping>>) {
    let seeds = input
        .lines()
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect::<Vec<isize>>();

    let mappings: Vec<Vec<Mapping>> = input
        .split("\n\n")
        .skip(1)
        .map(|mapping| {
            mapping
                .lines()
                .skip(1)
                .fold(Vec::new(), |mut acc, mapping_line| {
                    let (destination, source, length) = mapping_line
                        .split_whitespace()
                        .map(|val| val.parse::<isize>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    acc.push(Mapping {
                        range: source..source + length,
                        transformation: destination - source,
                    });
                    acc
                })
        })
        .collect();

    (seeds, mappings)
}

fn seedno(mappings: &[Vec<Mapping>], seed_value: isize) -> isize {
    let mut return_value = seed_value;
    for kind in mappings {
        for mapping in kind {
            if mapping.range.contains(&return_value) {
                return_value += mapping.transformation;
                break;
            }
        }
    }
    return_value
}

fn main() {
    let input = include_str!("../input.txt");
    let (seeds, mappings) = parse_input(input);

    let p1 = seeds.iter().fold(isize::MAX, |lowest, &seed| {
        lowest.min(seedno(&mappings, seed))
    });

    let p2 = seeds
        .iter()
        .tuples()
        .flat_map(|(&start, &length)| (start..start + length).map(|seed| seedno(&mappings, seed)))
        .min()
        .unwrap();

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
