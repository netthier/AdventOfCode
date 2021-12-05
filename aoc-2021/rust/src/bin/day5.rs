use std::cmp::{max, min};

fn main() {
    let input = include_str!("../../../inputs/day5");
    let parsed: Vec<Vec<usize>> = input
        .lines()
        .map(|e| {
            e.split(" -> ")
                .flat_map(|e| e.split(','))
                .map(|e| e.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let mut field = [[0_u8; 1000]; 1000];

    for line in &parsed {
        if line[0] == line[2] {
            for y in min(line[1], line[3])..=max(line[1], line[3]) {
                field[line[0]][y] += 1;
            }
        } else if line[1] == line[3] {
            for x in min(line[0], line[2])..=max(line[0], line[2]) {
                field[x][line[1]] += 1;
            }
        } else {
            let sign = if line[1] < line[3] { 1 } else { -1 };
            if line[0] < line[2] {
                for (idx, x) in (line[0]..=line[2]).enumerate() {
                    field[x][((line[1] as i32) + (idx as i32) * sign) as usize] += 8;
                }
            } else {
                for (idx, x) in (line[2]..=line[0]).enumerate() {
                    field[x][((line[3] as i32) + (idx as i32) * -sign) as usize] += 8;
                }
            }
        }
    }

    println!(
        "Part 1: {}",
        field
            .iter()
            .flat_map(|e| e.iter().filter(|e| **e % 8 > 1))
            .count()
    );

    println!(
        "Part 2: {}",
        field
            .iter()
            .flat_map(|e| e.iter().filter(|e| **e % 8 + **e / 8 > 1))
            .count()
    );
}
