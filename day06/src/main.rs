struct Race {
    time: usize,
    distance: usize,
}

fn solution(races: &[Race]) -> usize {
    races
        .iter()
        .map(|race| {
            (0..=race.time)
                .map(|hold_duration| (hold_duration * (race.time - hold_duration)))
                .filter(|&distance| distance > race.distance)
                .count()
        })
        .product::<usize>()
}

fn main() {
    let races = [
        Race {
            time: 60,
            distance: 475,
        },
        Race {
            time: 94,
            distance: 2138,
        },
        Race {
            time: 78,
            distance: 1015,
        },
        Race {
            time: 82,
            distance: 1650,
        },
    ];

    let p1 = solution(&races);
    let p2 = solution(&[Race {
        time: 60947882,
        distance: 475213810151650,
    }]);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
