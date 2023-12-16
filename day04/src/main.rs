use std::collections::HashSet;

fn main() {
    let wins: Vec<u32> = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let winning_nums = (&mut parts).skip(2).take(10).collect::<HashSet<_>>();
            let nums = (&mut parts).skip(1).collect::<HashSet<_>>();
            winning_nums.intersection(&nums).count() as u32
        })
        .collect();

    let p1 = wins
        .iter()
        .map(|&n_wins| {
            if n_wins == 0 {
                return 0;
            }
            2u32.pow(n_wins - 1)
        })
        .sum::<u32>();

    let p2 = wins
        .iter()
        .enumerate()
        .fold(vec![1; wins.len()], |mut card_counts, (idx, &n_wins)| {
            let card_amount = card_counts[idx];
            card_counts[idx + 1..=idx + n_wins as usize]
                .iter_mut()
                .for_each(|count| *count += card_amount);
            card_counts
        })
        .iter()
        .sum::<u32>();

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
