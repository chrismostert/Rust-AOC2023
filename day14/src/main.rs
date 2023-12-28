use std::collections::HashSet;

use grid::Grid;
use itertools::Itertools;

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

fn p1(platform: &Platform) -> usize {
    let mut plat = platform.clone();
    plat.tilt();
    plat.load_value()
}

fn p2(platform: &Platform) -> usize {
    let mut plat = platform.clone();
    
    let mut cycle_count = 0;
    let mut cycles_seen: HashSet<Platform> = HashSet::new();

    let to_reach = 1_000_000_000;

    loop {
        if !cycles_seen.contains(&plat) {
            cycles_seen.insert(plat.clone());
            plat.cycle();
            cycle_count += 1;
        } else {
            break;
        }
    }

    let cycle_start = cycle_count;
    let cycle_check = plat.clone();

    loop {
        plat.cycle();
        cycle_count += 1;
        if plat == cycle_check {
            break;
        }
    }

    let cycle_length = cycle_count - cycle_start;
    for _ in 0..((to_reach - cycle_count) % cycle_length) {
        plat.cycle();
    }

    plat.load_value()
}

fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let platform = Platform { grid: input.into() };

    let p1 = p1(&platform);
    let p2 = p2(&platform);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
