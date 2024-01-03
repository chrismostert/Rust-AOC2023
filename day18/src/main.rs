use itertools::Itertools;
use std::iter::once;

fn surface_area(coords: &[(isize, isize)], steps: isize) -> usize {
    (once(coords.last().unwrap())
        .chain(coords.iter())
        .chain(once(coords.first().unwrap()))
        .tuple_windows()
        .fold(0, |acc, (prev, cur, next)| {
            acc + (cur.0 * (next.1 - prev.1))
        })
        / 2)
    .unsigned_abs() as usize
        + (steps / 2) as usize
        + 1
}

fn main() {
    let (coords, hex_coords, steps, hex_steps, _, _) = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let (dir_str, steps, hex_str) = line.split_whitespace().collect_tuple().unwrap();
            (
                match dir_str {
                    "U" => (-1, 0),
                    "R" => (0, 1),
                    "D" => (1, 0),
                    "L" => (0, -1),
                    _ => unreachable!(),
                },
                steps.parse::<isize>().unwrap(),
                match &hex_str[7..=7] {
                    "0" => (0, 1),
                    "1" => (1, 0),
                    "2" => (0, -1),
                    "3" => (-1, 0),
                    _ => unreachable!(),
                },
                isize::from_str_radix(&hex_str[2..7], 16).unwrap(),
            )
        })
        .fold(
            (Vec::new(), Vec::new(), 0, 0, (0, 0), (0, 0)),
            |(mut coords, mut hex_coords, steps_total, hex_steps_total, cur, hex_cur),
             (dir, steps, hex_dir, hex_steps)| {
                let (new_cur, new_hex_cur) = (
                    (cur.0 + dir.0 * steps, cur.1 + dir.1 * steps),
                    (
                        hex_cur.0 + hex_dir.0 * hex_steps,
                        hex_cur.1 + hex_dir.1 * hex_steps,
                    ),
                );
                coords.push(new_cur);
                hex_coords.push(new_hex_cur);
                (
                    coords,
                    hex_coords,
                    steps_total + steps,
                    hex_steps_total + hex_steps,
                    new_cur,
                    new_hex_cur,
                )
            },
        );

    let (p1, p2) = (
        surface_area(&coords, steps),
        surface_area(&hex_coords, hex_steps),
    );
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
