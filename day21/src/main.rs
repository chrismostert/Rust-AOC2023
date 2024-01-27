use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug)]
struct Grid {
    items: Vec<Vec<char>>,
}

impl Grid {
    fn neighbor_coords(&self, (row, col): (usize, usize)) -> Vec<(usize, usize)> {
        [
            (row.saturating_sub(1), col),
            (row + 1, col),
            (row, col.saturating_sub(1)),
            (row, col + 1),
        ]
        .into_iter()
        .filter(|&(neigh_row, neigh_col)| {
            (neigh_row, neigh_col) != (row, col)
                && matches!(
                    self.items.get(neigh_row).and_then(|r| r.get(neigh_col)),
                    Some('.') | Some('S')
                )
        })
        .collect_vec()
    }

    fn reachable_in_n_steps(&self, start_pos: (usize, usize), n: usize) -> HashSet<(usize, usize)> {
        let mut current_positions = HashSet::from([start_pos]);
        let mut next_positions = HashSet::new();

        for _ in 0..n {
            next_positions.clear();
            current_positions.into_iter().for_each(|pos| {
                self.neighbor_coords(pos).into_iter().for_each(|new_pos| {
                    next_positions.insert(new_pos);
                })
            });
            current_positions = next_positions.clone();
        }

        current_positions
    }
}

fn parse_input(input: &str) -> Grid {
    Grid {
        items: input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect_vec(),
    }
}

fn main() {
    let grid = parse_input(include_str!("../input.txt"));
    let start_pos = (
        (grid.items.len() - 1) / 2,
        (grid.items.get(0).unwrap().len() - 1) / 2,
    );

    let p1 = grid.reachable_in_n_steps(start_pos, 64).len();
    // Hand calculated, 26501365 steps is 202300*131 + 65 steps.
    // Thus we get the step counts for 65, 196 and 327 and from the step counts we see
    // that it results in a quadratic formula.
    let p2: usize = 14861 * (202301 * 202301) - 14728 * 202301 + 3658;

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
