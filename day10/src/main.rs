use helpers::CharGrid;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

type Coord = (i32, i32);
type CoordMap = HashMap<Coord, Vec<Coord>>;

fn get_directions(piece: &char, (x, y): Coord) -> Vec<Coord> {
    match piece {
        '|' => vec![(x, y - 1), (x, y + 1)],
        '-' => vec![(x - 1, y), (x + 1, y)],
        'L' => vec![(x, y - 1), (x + 1, y)],
        'J' => vec![(x, y - 1), (x - 1, y)],
        '7' => vec![(x - 1, y), (x, y + 1)],
        'F' => vec![(x + 1, y), (x, y + 1)],
        'S' => vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)],
        _ => vec![],
    }
}

fn get_loop(coord_map: &CoordMap, start: Coord) -> Vec<Coord> {
    let mut visited = HashSet::from([start]);
    let mut path = vec![start];
    let mut position = start;

    while let Some(pos_new) = coord_map[&position]
        .iter()
        .find(|pos| !visited.contains(pos))
    {
        visited.insert(*pos_new);
        path.push(*pos_new);
        position = *pos_new;
    }

    path
}

// Shoelace formula
fn surface_area(coords: &[Coord]) -> usize {
    let length = coords.len();
    (once(length - 1)
        .chain(0..length)
        .chain(once(0))
        .tuple_windows()
        .fold(0, |acc, (prev, cur, next)| {
            acc + (coords[cur].0 * (coords[next].1 - coords[prev].1))
        })
        / 2)
    .unsigned_abs() as usize
}

fn main() {
    let input = include_str!("../input.txt").parse::<CharGrid>().unwrap();
    let coord_map: CoordMap = (0..input.width).cartesian_product(0..input.height).fold(
        HashMap::new(),
        |mut map, (x_source, y_source)| {
            let possibilities = get_directions(
                &input.get(x_source, y_source).unwrap(),
                (x_source, y_source),
            );
            let reachable = possibilities
                .iter()
                .filter(|(x_dest, y_dest)| {
                    if let Some(c) = input.get(*x_dest, *y_dest) {
                        return get_directions(&c, (*x_dest, *y_dest))
                            .contains(&(x_source, y_source));
                    }
                    false
                })
                .copied()
                .collect_vec();

            map.insert((x_source, y_source), reachable);
            map
        },
    );
    let start_coord = (0..input.width)
        .cartesian_product(0..input.height)
        .find(|(x, y)| input.get(*x, *y) == Some('S'))
        .unwrap();

    let loop_coords = get_loop(&coord_map, start_coord);
    let p1 = loop_coords.len() / 2;

    // Pick's theorem
    let p2 = surface_area(&loop_coords) - p1 + 1;

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
