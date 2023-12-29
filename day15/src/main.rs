use indexmap::IndexMap;
use itertools::Itertools;

fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

fn main() {
    let input = include_str!("../input.txt").trim().split(',').collect_vec();

    let p1 = input.iter().copied().map(hash).sum::<usize>();
    let p2 = input
        .iter()
        .fold(
            vec![IndexMap::<&str, usize>::new(); 256],
            |mut boxes, inp| {
                if inp.contains('-') {
                    let label = &inp[0..inp.len() - 1];
                    boxes[hash(label)] =
                        boxes[hash(label)]
                            .iter()
                            .fold(IndexMap::new(), |mut acc, (&lab, value)| {
                                if label != lab {
                                    acc.insert(lab, *value);
                                }
                                acc
                            })
                } else {
                    let (label, value) = inp.split_once('=').unwrap();
                    boxes[hash(label)].insert(label, value.parse().unwrap());
                }
                boxes
            },
        )
        .iter()
        .enumerate()
        .map(|(box_idx, box_elem)| {
            box_elem
                .iter()
                .enumerate()
                .map(move |(lens_idx, (_, focal_length))| {
                    (box_idx + 1) * (lens_idx + 1) * focal_length
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
