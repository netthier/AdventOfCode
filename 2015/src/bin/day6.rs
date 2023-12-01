fn main() {
    let input = include_str!("../../inputs/day6").trim();
    let parsed = input
        .lines()
        .map(|e| {
            let mut words = e.split_whitespace();
            let state = match words.next().unwrap() {
                "turn" => match words.next().unwrap() {
                    "on" => 1,
                    "off" => 0,
                    _ => unreachable!(),
                },
                "toggle" => 2,
                _ => unreachable!(),
            };

            let coords = words.map(|e| e.split(',').filter_map(|e| e.parse::<usize>().ok())).flatten().collect();

            (state, coords)
        }).collect::<Vec<(usize, Vec<usize>)>>();

    let mut part_one = vec![vec![false; 1000]; 1000];
    let mut part_two = vec![vec![0_usize; 1000]; 1000];

    parsed.iter().for_each(|(state, coords)|
        for x in coords[0]..=coords[2] {
            for y in coords[1]..=coords[3] {
                part_one[x][y] = match state {
                    0 => false,
                    1 => true,
                    2 => !part_one[x][y],
                    _ => unreachable!(),
                };

                part_two[x][y] = match state {
                    0 => part_two[x][y].saturating_sub(1),
                    1 => part_two[x][y] + 1,
                    2 => part_two[x][y] + 2,
                    _ => unreachable!(),
                };
            }
        }
    );

    println!("Part 1: {}", part_one.iter().flatten().filter(|e| **e).count());
    println!("Part 2: {}", part_two.iter().flatten().sum::<usize>());
}

