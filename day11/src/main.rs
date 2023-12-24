use helpers::CharGrid;
use itertools::Itertools;
use std::collections::HashSet;

enum Direction {
    Row,
    Column,
}

fn empty_indexes(image: &CharGrid, direction: Direction) -> HashSet<i32> {
    (0..image.width)
        .filter(|&a| {
            (0..image.height)
                .map(|b| match direction {
                    Direction::Row => image.get(b, a).unwrap(),
                    Direction::Column => image.get(a, b).unwrap(),
                })
                .all(|c| c == '.')
        })
        .collect()
}

fn galaxy_dist(
    (x_from, y_from): (i32, i32),
    (x_to, y_to): (i32, i32),
    empty_rows: &HashSet<i32>,
    empty_cols: &HashSet<i32>,
    expansions: usize,
) -> usize {
    let x_dist = (x_to - x_from).unsigned_abs() as usize;
    let y_dist = (y_to - y_from).unsigned_abs() as usize;

    x_dist
        + y_dist
        + [
            (x_from, x_to, x_dist, empty_cols),
            (y_from, y_to, y_dist, empty_rows),
        ]
        .iter()
        .map(|&(from, to, dist, cols)| {
            (from.min(to)..)
                .take(dist + 1)
                .filter(|idx| cols.contains(idx))
                .count()
                * expansions
        })
        .sum::<usize>()
}

fn solution(
    galaxies: &[(i32, i32)],
    empty_rows: &HashSet<i32>,
    empty_cols: &HashSet<i32>,
    expansion_multiplier: usize,
) -> usize {
    galaxies
        .iter()
        .tuple_combinations()
        .map(|(from, to)| galaxy_dist(*from, *to, empty_rows, empty_cols, expansion_multiplier - 1))
        .sum::<usize>()
}

fn main() {
    let image = include_str!("../input.txt").parse::<CharGrid>().unwrap();
    let galaxies = image.find_coords(&'#');

    let empty_rows = empty_indexes(&image, Direction::Row);
    let empty_cols = empty_indexes(&image, Direction::Column);

    let p1 = solution(&galaxies, &empty_rows, &empty_cols, 2);
    let p2 = solution(&galaxies, &empty_rows, &empty_cols, 1_000_000);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
