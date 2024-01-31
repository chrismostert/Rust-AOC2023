use itertools::Itertools;
use std::{
    cell::{Cell, RefCell},
    collections::{HashSet, VecDeque},
};

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

struct Brick<'a> {
    id: usize,
    top_z: Cell<usize>,
    bricks_below: RefCell<Vec<&'a Brick<'a>>>,
    bricks_above: RefCell<Vec<&'a Brick<'a>>>,
    p1: Point,
    p2: Point,
}
impl Brick<'_> {
    fn from(line: &str, id: usize) -> Self {
        let (p1, p2) = line.split('~').map(Point::from).collect_tuple().unwrap();
        Brick {
            id,
            top_z: Cell::new(p2.z),
            bricks_below: RefCell::new(Vec::new()),
            bricks_above: RefCell::new(Vec::new()),
            p1,
            p2,
        }
    }
    fn coords(&self) -> impl Iterator<Item = (usize, usize)> {
        (self.p1.x..=self.p2.x).cartesian_product(self.p1.y..=self.p2.y)
    }
}

fn fall_bricks<'a>(bricks: &'a [Brick<'a>]) {
    let mut top_bricks: [[Option<&'a Brick>; 10]; 10] = [[None; 10]; 10];

    for falling_brick in bricks.iter() {
        let bricks_we_fall_on = falling_brick
            .coords()
            .filter_map(|(x, y)| top_bricks[y][x])
            .unique_by(|b| b.id)
            .max_set_by_key(|&brick| brick.top_z.get());

        for &brick in bricks_we_fall_on.iter() {
            falling_brick.bricks_below.borrow_mut().push(brick);
            brick.bricks_above.borrow_mut().push(falling_brick);
        }

        falling_brick.top_z.set(
            bricks_we_fall_on
                .iter()
                .map(|b| b.top_z.get())
                .next()
                .unwrap_or(0)
                + 1
                + (falling_brick.p2.z - falling_brick.p1.z),
        );

        for (x, y) in falling_brick.coords() {
            top_bricks[y][x] = Some(falling_brick);
        }
    }
}

fn n_safe_bricks(bricks: &[Brick]) -> usize {
    bricks
        .iter()
        .filter(|brick| {
            brick.bricks_above.borrow().len() == 0
                || brick
                    .bricks_above
                    .borrow()
                    .iter()
                    .all(|above| above.bricks_below.borrow().len() > 1)
        })
        .count()
}

fn n_falling_bricks(brick_to_remove: &Brick) -> usize {
    let mut to_check: VecDeque<&Brick> = brick_to_remove
        .bricks_above
        .borrow()
        .iter()
        .copied()
        .collect();
    let mut fallen = HashSet::from([brick_to_remove.id]);

    while let Some(brick) = to_check.pop_front() {
        if brick
            .bricks_below
            .borrow()
            .iter()
            .filter(|b| !fallen.contains(&b.id))
            .count()
            == 0
        {
            fallen.insert(brick.id);
            brick
                .bricks_above
                .borrow()
                .iter()
                .for_each(|&b| to_check.push_back(b));
        }
    }

    fallen.len() - 1
}

fn main() {
    let bricks = include_str!("../input.txt")
        .lines()
        .enumerate()
        .map(|(idx, line)| Brick::from(line, idx))
        .sorted_by(|a, b| a.p1.z.cmp(&b.p1.z))
        .collect_vec();

    fall_bricks(&bricks);

    let p1 = n_safe_bricks(&bricks);
    let p2 = bricks.iter().map(n_falling_bricks).sum::<usize>();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
