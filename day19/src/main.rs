mod containers;
use containers::{Operator, Output, Part, RangeValue, Ranges, Rule, Workflow};
use itertools::Itertools;
use std::collections::HashMap;

fn str_to_output(str: &str) -> Output {
    match str {
        "A" => Output::Accept,
        "R" => Output::Reject,
        redirect => Output::Redirect(String::from(redirect)),
    }
}
fn str_to_operator(str: &str) -> Operator {
    match str {
        "A" => Operator::Return(str_to_output(str)),
        "R" => Operator::Return(str_to_output(str)),
        redirect => Operator::Return(str_to_output(redirect)),
    }
}
fn parse_input(input: &str) -> (HashMap<&str, Workflow>, Vec<Part>) {
    let (workflows_str, parts_str) = input
        .split("\n\n")
        .map(|part| part.trim())
        .collect_tuple()
        .unwrap();

    let workflows = workflows_str
        .lines()
        .map(|line| {
            let (name, rest_str) = line.split_once('{').unwrap();
            (
                name,
                Workflow {
                    rules: rest_str
                        .strip_suffix('}')
                        .unwrap()
                        .split(',')
                        .map(|rule| {
                            if !rule.contains(':') {
                                return Rule {
                                    operation: str_to_operator(rule),
                                };
                            }
                            let (comp_string, result) = rule.split_once(':').unwrap();
                            let field = &comp_string[0..=0];
                            let operation = match &comp_string[1..=1] {
                                ">" => Operator::Greater(
                                    field,
                                    (comp_string[2..]).parse().unwrap(),
                                    str_to_output(result),
                                ),
                                "<" => Operator::Less(
                                    field,
                                    (comp_string[2..]).parse().unwrap(),
                                    str_to_output(result),
                                ),
                                _ => unreachable!(),
                            };
                            Rule { operation }
                        })
                        .collect_vec(),
                },
            )
        })
        .collect::<HashMap<_, _>>();

    let parts = parts_str
        .lines()
        .map(|line| {
            let values = line
                .split('=')
                .skip(1)
                .map(|part| {
                    part.split_once([',', '}'])
                        .unwrap()
                        .0
                        .parse::<usize>()
                        .unwrap()
                })
                .collect_vec();
            Part {
                x: values[0],
                m: values[1],
                a: values[2],
                s: values[3],
            }
        })
        .collect_vec();

    (workflows, parts)
}

fn run_parts_through_workflows(workflows: &HashMap<&str, Workflow>, part: &Part) -> Output {
    let mut result = workflows["in"].run_on(part);
    while let Output::Redirect(redirect) = result {
        result = workflows[&redirect[..]].run_on(part);
    }
    result
}

fn amount_of_accepted_parts(workflows: &HashMap<&str, Workflow>) -> usize {
    let mut frontier: Vec<(Workflow, Ranges)> = vec![(
        workflows["in"].clone(),
        Ranges {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        },
    )];
    let mut valid_ranges: Vec<Ranges> = Vec::new();

    while let Some((workflow, ranges)) = frontier.pop() {
        let mut current_ranges = ranges;
        for rule in workflow.rules {
            match rule.operation {
                Operator::Return(Output::Accept) => valid_ranges.push(current_ranges.clone()),
                Operator::Return(Output::Redirect(redirect)) => {
                    frontier.push((workflows[&redirect[..]].clone(), current_ranges.clone()))
                }
                Operator::Return(Output::Reject) => (),
                Operator::Less(field, value, ret) => {
                    match ret {
                        Output::Accept => {
                            valid_ranges.push(current_ranges.set(
                                field,
                                RangeValue::Upper,
                                value - 1,
                            ));
                        }
                        Output::Redirect(redirect) => frontier.push((
                            workflows[&redirect[..]].clone(),
                            current_ranges.set(field, RangeValue::Upper, value - 1),
                        )),
                        Output::Reject => (),
                    }
                    current_ranges = current_ranges.set(field, RangeValue::Lower, value);
                }
                Operator::Greater(field, value, ret) => {
                    match ret {
                        Output::Accept => {
                            valid_ranges.push(current_ranges.set(
                                field,
                                RangeValue::Lower,
                                value + 1,
                            ));
                        }
                        Output::Redirect(redirect) => frontier.push((
                            workflows[&redirect[..]].clone(),
                            current_ranges.set(field, RangeValue::Lower, value + 1),
                        )),
                        Output::Reject => (),
                    }
                    current_ranges = current_ranges.set(field, RangeValue::Upper, value);
                }
            }
        }
    }
    valid_ranges
        .iter()
        .map(|ranges| ranges.get_valid_amount())
        .sum()
}

fn main() {
    let (workflows, parts) = parse_input(include_str!("../input.txt"));
    let p1 = parts
        .into_iter()
        .filter(|part| run_parts_through_workflows(&workflows, part) == Output::Accept)
        .map(|part| part.x + part.m + part.a + part.s)
        .sum::<usize>();
    let p2 = amount_of_accepted_parts(&workflows);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
