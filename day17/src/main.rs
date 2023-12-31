use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};
#[derive(PartialEq, Eq)]
struct State {
    position: (usize, usize),
    direction: (isize, isize),
    cost: usize,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.cost.cmp(&other.cost) {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
        }
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl State {
    fn next_states(&self, grid: &[Vec<usize>], min_steps: usize, max_steps: usize) -> Vec<State> {
        match self.direction {
            (1, 0) | (-1, 0) => [(0, 1), (0, -1)],
            (0, 1) | (0, -1) => [(1, 0), (-1, 0)],
            _ => unreachable!(),
        }
        .iter()
        .flat_map(|&dir| {
            (1..=max_steps)
                .map(move |steps| {
                    (
                        self.position.0 as isize + steps as isize * dir.0,
                        self.position.1 as isize + steps as isize * dir.1,
                    )
                })
                .filter(|&(row, col)| {
                    row >= 0
                        && col >= 0
                        && row < grid.len() as isize
                        && col < grid[0].len() as isize
                })
                .enumerate()
                .fold(
                    (self.cost, Vec::new()),
                    |(running_cost, mut states), (step_no, (row, col))| {
                        if step_no + 1 >= min_steps {
                            states.push(State {
                                position: (row as usize, col as usize),
                                direction: dir,
                                cost: running_cost + grid[row as usize][col as usize],
                            });
                        }
                        (running_cost + grid[row as usize][col as usize], states)
                    },
                )
                .1
        })
        .collect_vec()
    }
}

fn dijkstra(grid: &[Vec<usize>], min_steps: usize, max_steps: usize) -> Option<usize> {
    let mut dists: HashMap<((usize, usize), (isize, isize)), usize> =
        HashMap::from([(((0, 0), (0, 1)), 0), (((0, 0), (1, 0)), 0)]);
    let mut frontier = BinaryHeap::from([
        State {
            position: (0, 0),
            direction: (0, 1),
            cost: 0,
        },
        State {
            position: (0, 0),
            direction: (1, 0),
            cost: 0,
        },
    ]);

    while let Some(state) = frontier.pop() {
        if state.position == (grid.len() - 1, grid[0].len() - 1) {
            return Some(state.cost);
        }
        if state.cost
            > *dists
                .get(&(state.position, state.direction))
                .unwrap_or(&usize::MAX)
        {
            continue;
        }
        for neighbor in state.next_states(grid, min_steps, max_steps) {
            if neighbor.cost
                < *dists
                    .get(&(neighbor.position, neighbor.direction))
                    .unwrap_or(&usize::MAX)
            {
                dists.insert((neighbor.position, neighbor.direction), neighbor.cost);
                frontier.push(neighbor);
            }
        }
    }
    None
}

fn main() {
    let grid = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect_vec();

    let p1 = dijkstra(&grid, 1, 3).unwrap();
    let p2 = dijkstra(&grid, 4, 10).unwrap();

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
