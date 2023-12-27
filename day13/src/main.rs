use helpers::CharGrid;
use itertools::Itertools;

enum Line {
    Horizontal(usize),
    Vertical(usize),
}

fn find_mirror_line(grid: &CharGrid, diffs_needed: usize) -> Option<Line> {
    for col_no in 1..grid.width {
        let mut diffs = 0;
        for (col_l, col_r) in (0..col_no).rev().zip(col_no..grid.width) {
            for y in 0..grid.height {
                if grid.get(col_l, y).unwrap() != grid.get(col_r, y).unwrap() {
                    diffs += 1;
                }
            }
        }
        if diffs == diffs_needed {
            return Some(Line::Vertical(col_no as usize));
        }
    }

    for row_no in 1..grid.height {
        let mut diffs = 0;
        for (row_t, row_b) in (0..row_no).rev().zip(row_no..grid.height) {
            for x in 0..grid.width {
                if grid.get(x, row_t).unwrap() != grid.get(x, row_b).unwrap() {
                    diffs += 1;
                }
            }
        }
        if diffs == diffs_needed {
            return Some(Line::Horizontal(row_no as usize));
        }
    }

    None
}

fn line_to_val(line: Line) -> usize {
    match line {
        Line::Horizontal(val) => 100 * val,
        Line::Vertical(val) => val,
    }
}

fn solution(grids: &[CharGrid], diffs_needed: usize) -> usize {
    grids
        .iter()
        .filter_map(|grid| find_mirror_line(grid, diffs_needed))
        .map(line_to_val)
        .sum::<usize>()
}

fn main() {
    let grids = include_str!("../input.txt")
        .split("\n\n")
        .map(|grid| grid.parse::<CharGrid>().unwrap())
        .collect_vec();

    let p1 = solution(&grids, 0);
    let p2 = solution(&grids, 1);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
