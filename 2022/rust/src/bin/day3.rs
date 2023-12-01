fn main() {
    let input = include_str!("../../../inputs/day3");
    let contents: Vec<&str> = input.lines().collect();
    let mut part_one = 0;
    for pair in contents.iter().map(|e| e.split_at(e.len() / 2)) {
        for c in pair.0.chars() {
            if pair.1.contains(c) {
                part_one += to_priority(c) as u32;
                break;
            }
        }
    }

    println!("Part 1: {part_one}");

    let mut part_two = 0;
    for group in contents.chunks(3) {
        for c in group[0].chars() {
            if group[1].contains(c) && group[2].contains(c) {
                part_two += to_priority(c) as u32;
                break;
            }
        }
    }

    println!("Part 2: {part_two}");
}

fn to_priority(c: char) -> u8 {
    if c.is_uppercase() {
        c as u8 - 38
    } else {
        c as u8 - 96
    }
}
