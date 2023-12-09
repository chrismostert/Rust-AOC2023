type Reveal = (u32, u32, u32);
struct Game {
    id: u32,
    reveals: Vec<Reveal>,
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(id, reveals)| Game {
            id: id.split_once(' ').unwrap().1.parse().unwrap(),
            reveals: reveals
                .split("; ")
                .map(|reveal_group| {
                    reveal_group
                        .split(", ")
                        .fold((0, 0, 0), |(r, g, b), reveal| match reveal
                            .split_once(' ')
                            .unwrap()
                        {
                            (val, "red") => (val.parse().unwrap(), g, b),
                            (val, "green") => (r, val.parse().unwrap(), b),
                            (val, "blue") => (r, g, val.parse().unwrap()),
                            _ => unreachable!(),
                        })
                })
                .collect(),
        })
        .collect()
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));

    let p1: u32 = input
        .iter()
        .filter(|game| {
            game.reveals
                .iter()
                .all(|&(r, g, b)| r <= 12 && g <= 13 && b <= 14)
        })
        .map(|game| game.id)
        .sum();

    let p2: u32 = input
        .iter()
        .map(|game| {
            game.reveals
                .iter()
                .fold((0, 0, 0), |(r, g, b), &(rc, gc, bc)| {
                    (r.max(rc), g.max(gc), b.max(bc))
                })
        })
        .map(|(r, g, b)| r * g * b)
        .sum();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
