use grid::Grid;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Platform {
    grid: Grid<char>,
}

impl Platform {
    fn tilt(&mut self) {
        for (y, x) in (0..self.grid.rows()).cartesian_product(0..self.grid.cols()) {
            if self.grid.get(y, x) == Some(&'O') {
                let mut roll_to = y;
                while roll_to > 0 && self.grid.get(roll_to - 1, x) == Some(&'.') {
                    roll_to -= 1;
                }
                *self.grid.get_mut(y, x).unwrap() = '.';
                *self.grid.get_mut(roll_to, x).unwrap() = 'O';
            }
        }
    }

    fn cycle(&mut self) {
        for _ in 0..4 {
            self.tilt();
            self.grid.rotate_right();
        }
    }

    fn load_value(&self) -> usize {
        (0..self.grid.rows())
            .cartesian_product(0..self.grid.cols())
            .fold(0, |acc, (y, x)| {
                if self.grid.get(y, x) == Some(&'O') {
                    return acc + self.grid.rows() - y;
                }
                acc
            })
    }
}

fn p1(mut platform: Platform) -> usize {
    platform.tilt();
    platform.load_value()
}

fn p2(mut platform: Platform) -> usize {
    let mut hash_to_iter_no: HashMap<Platform, usize> = HashMap::new();
    let mut iter_no = 0;

    while hash_to_iter_no.get(&platform).is_none() {
        hash_to_iter_no.insert(platform.clone(), iter_no);
        platform.cycle();
        iter_no += 1;
    }

    for _ in 0..(1_000_000_000 - iter_no) % (iter_no - hash_to_iter_no.get(&platform).unwrap()) {
        platform.cycle();
    }

    platform.load_value()
}

fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let platform = Platform { grid: input.into() };

    let p1 = p1(platform.clone());
    let p2 = p2(platform);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
