use helpers::CharGrid;
use itertools::Itertools;
use std::collections::VecDeque;

fn full_digit(grid: &CharGrid, x: i32, y: i32) -> Option<u32> {
    let mut found_digits = VecDeque::from([grid.get(x, y)?.to_digit(10)?]);

    let mut left_digits = (0..x)
        .rev()
        .map(|left_idx| grid.get(left_idx, y).and_then(|val| val.to_digit(10)));
    while let Some(Some(digit)) = left_digits.next() {
        found_digits.push_front(digit);
    }

    let mut right_digits =
        (x + 1..).map(|right_idx| grid.get(right_idx, y).and_then(|val| val.to_digit(10)));
    while let Some(Some(digit)) = right_digits.next() {
        found_digits.push_back(digit);
    }

    let base: u32 = 10;
    Some(found_digits.iter().enumerate().fold(0, |acc, (idx, val)| {
        acc + val * (base.pow((found_digits.len() - 1 - idx) as u32))
    }))
}

fn main() {
    let grid: CharGrid = include_str!("../input.txt").parse().unwrap();
    let mut digits: Vec<u32> = Vec::new();
    let mut gear_ratios: Vec<u32> = Vec::new();

    for (x, y) in (0..grid.width).cartesian_product(0..grid.height) {
        let elem = grid.get(x, y).unwrap();
        if elem != '.' && !elem.is_numeric() {
            let mut neighbours: Vec<u32> = grid
                .get_neighbours(x, y)
                .iter()
                .filter_map(|((xd, yd), _)| full_digit(&grid, *xd, *yd))
                .unique()
                .collect();

            if elem == '*' && neighbours.len() == 2 {
                gear_ratios.push(neighbours[0] * neighbours[1]);
            }
            digits.append(&mut neighbours);
        }
    }

    let p1 = digits.iter().sum::<u32>();
    let p2 = gear_ratios.iter().sum::<u32>();

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
