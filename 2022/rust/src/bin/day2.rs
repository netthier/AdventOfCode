fn main() {
    let input = include_str!("../../../inputs/day2");
    let pairs: Vec<(char, char)> = input
        .lines()
        .map(|e| {
            let mut c = e.chars();
            (c.next().unwrap(), c.nth(1).unwrap())
        })
        .collect();

    println!("Part 1: {}", pairs.iter().map(part_one).sum::<u32>());
    println!("Part 2: {}", pairs.iter().map(part_two).sum::<u32>());
}

fn part_one((a, b): &(char, char)) -> u32 {
    let au = *a as u8 - 64;
    let bu = *b as u8 - 87;
    bu as u32
        + match (a, b) {
            ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
            ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
            _ if au == bu => 3,
            _ => panic!(),
        }
}

fn part_two((a, b): &(char, char)) -> u32 {
    let res = match b {
        'X' => (*a as i8 - 66).rem_euclid(3) + 1,
        'Y' => (*a as i8) - 64,
        'Z' => (*a as i8 - 64).rem_euclid(3) + 1,
        _ => panic!(),
    } as u8;
    (res + (*b as u8 - 88) * 3) as u32
}
