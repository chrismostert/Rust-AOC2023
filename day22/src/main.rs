use itertools::Itertools;
use std::collections::{HashMap, HashSet};

struct Point {
    x: usize,
    y: usize,
    z: usize,
}
impl Point {
    fn from(text: &str) -> Self {
        let (x, y, z) = text
            .split(',')
            .map(|d| d.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        Point { x, y, z }
    }
}

struct Brick {
    id: usize,
    p1: Point,
    p2: Point,
}
impl Brick {
    fn from(line: &str, id: usize) -> Self {
        let (p1, p2) = line.split('~').map(Point::from).collect_tuple().unwrap();
        Brick { id, p1, p2 }
    }
    fn coords(&self) -> impl Iterator<Item = (usize, usize)> {
        (self.p1.x..=self.p2.x).cartesian_product(self.p1.y..=self.p2.y)
    }
}

type SupportMap = HashMap<usize, HashSet<usize>>;
fn construct_support_graphs(bricks: &[Brick]) -> (SupportMap, SupportMap) {
    let mut top_bricks: [[Option<(usize, usize)>; 10]; 10] = [[None; 10]; 10];
    let mut supported_by = (0..bricks.len())
        .map(|idx| (idx, HashSet::new()))
        .collect::<SupportMap>();
    let mut supports = supported_by.clone();

    for falling_brick in bricks.iter() {
        let mut z_to_fall_to = 1;
        let mut possible_supports: Vec<(usize, usize)> = Vec::new();

        // Find highest intersection
        for (x, y) in falling_brick.coords() {
            if let Some((possible_support, height)) = top_bricks[y][x] {
                z_to_fall_to = z_to_fall_to.max(height + 1);
                possible_supports.push((possible_support, height));
            }
        }

        // Add to support graph
        for supporting_idx in possible_supports
            .into_iter()
            .filter(|&(_, height)| height == z_to_fall_to - 1)
            .map(|(brick_idx, _)| brick_idx)
            .unique()
        {
            supported_by
                .entry(falling_brick.id)
                .or_default()
                .insert(supporting_idx);
            supports
                .entry(supporting_idx)
                .or_default()
                .insert(falling_brick.id);
        }

        // Update top_bricks
        for (x, y) in falling_brick.coords() {
            top_bricks[y][x] = Some((
                falling_brick.id,
                z_to_fall_to + (falling_brick.p2.z - falling_brick.p1.z),
            ));
        }
    }

    (supported_by, supports)
}

fn unsafe_bricks(supported_by: &SupportMap) -> impl Iterator<Item = &usize> {
    supported_by
        .values()
        .filter(|vals| vals.len() == 1)
        .flatten()
        .unique()
}

fn n_safe_bricks(supported_by: &SupportMap, bricks: &[Brick]) -> usize {
    bricks.len() - unsafe_bricks(supported_by).count()
}

fn main() {
    let bricks = include_str!("../input.txt")
        .lines()
        .enumerate()
        .map(|(idx, line)| Brick::from(line, idx))
        .sorted_by(|a, b| a.p1.z.cmp(&b.p1.z))
        .collect_vec();

    let (supported_by, supports) = construct_support_graphs(&bricks);
    let p1 = n_safe_bricks(&supported_by, &bricks);

    println!("Part 1: {}", p1);
}
