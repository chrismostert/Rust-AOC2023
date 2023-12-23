use itertools::Itertools;

fn extrapolate(nums: &[isize], value: isize) -> isize {
    if nums.iter().tuple_windows().all(|(a, b)| a == b) {
        return value + *nums.last().unwrap();
    }
    extrapolate(
        &nums
            .iter()
            .tuple_windows()
            .map(|(&a, &b)| b - a)
            .collect_vec(),
        value + *nums.last().unwrap(),
    )
}

fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|digit| digit.parse::<isize>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let p1 = input.iter().map(|nums| extrapolate(nums, 0)).sum::<isize>();
    let p2 = input
        .iter()
        .map(|nums| extrapolate(&nums.iter().rev().copied().collect_vec(), 0))
        .sum::<isize>();

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
