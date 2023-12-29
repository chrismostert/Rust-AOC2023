use itertools::Itertools;
use std::collections::HashSet;

fn new_pos(grid: &[Vec<char>], pos: (usize, usize), dir: (isize, isize)) -> Option<(usize, usize)> {
    let new_pos = (pos.0 as isize + dir.0, pos.1 as isize + dir.1);
    if (new_pos.0 < 0)
        || (new_pos.1 < 0)
        || (new_pos.0 == grid.len() as isize)
        || (new_pos.1 == grid[0].len() as isize)
    {
        return None;
    }
    Some((new_pos.0 as usize, new_pos.1 as usize))
}

fn energize(grid: &[Vec<char>], entry: ((usize, usize), (isize, isize))) -> usize {
    let mut frontier = Vec::from([entry]);
    let mut visited = HashSet::new();

    while let Some(((row, col), dir)) = frontier.pop() {
        if !visited.insert(((row, col), dir)) {
            continue;
        }

        let new_dirs = match grid[row][col] {
            '.' => vec![dir],
            '-' => match dir {
                (0, 1) | (0, -1) => vec![dir],
                _ => vec![(0, -1), (0, 1)],
            },
            '|' => match dir {
                (-1, 0) | (1, 0) => vec![dir],
                _ => vec![(-1, 0), (1, 0)],
            },
            '/' => match dir {
                (0, 1) => vec![(-1, 0)],
                (0, -1) => vec![(1, 0)],
                (1, 0) => vec![(0, -1)],
                _ => vec![(0, 1)],
            },
            '\\' => match dir {
                (0, 1) => vec![(1, 0)],
                (0, -1) => vec![(-1, 0)],
                (1, 0) => vec![(0, 1)],
                _ => vec![(0, -1)],
            },
            _ => vec![(0, 0)],
        };

        for new_dir in new_dirs {
            if let Some((new_row, new_col)) = new_pos(grid, (row, col), new_dir) {
                frontier.push(((new_row, new_col), new_dir));
            }
        }
    }

    visited.iter().map(|(pos, _)| pos).unique().count()
}

fn main() {
    let grid: Vec<Vec<char>> = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let p1 = energize(&grid, ((0, 0), (0, 1)));

    let mut p2 = 0;
    for row in 0..grid.len() {
        p2 = p2.max(energize(&grid, ((row, 0), (0, 1))));
        p2 = p2.max(energize(&grid, ((row, grid[0].len() - 1), (0, -1))));
    }
    for col in 0..grid[0].len() {
        p2 = p2.max(energize(&grid, ((0, col), (1, 0))));
        p2 = p2.max(energize(&grid, ((grid.len() - 1, col), (-1, 0))));
    }

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
