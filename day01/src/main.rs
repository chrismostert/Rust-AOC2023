fn get_calibration(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.chars().find_map(|c| c.to_digit(10)).unwrap() * 10
                + line.chars().rev().find_map(|c| c.to_digit(10)).unwrap()
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let p1 = get_calibration(input);
    let p2 = get_calibration(
        &[
            ("one", "o1e"),
            ("two", "t2o"),
            ("three", "t3e"),
            ("four", "f4r"),
            ("five", "f5e"),
            ("six", "s6x"),
            ("seven", "s7n"),
            ("eight", "e8t"),
            ("nine", "n9e"),
        ]
        .iter()
        .fold(input.to_owned(), |s, (from, to)| s.replace(from, to)),
    );

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
