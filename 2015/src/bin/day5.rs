#![feature(option_result_contains)]

fn main() {
    let input = include_str!("../../inputs/day5").trim();

    let part_one = input
        .lines()
        .filter(|e| {
            e.chars().filter(|e| "aeiou".contains(*e)).count() >= 3
                && e.as_bytes().windows(2).any(|e| e[0] == e[1])
                && !e.contains("ab")
                && !e.contains("cd")
                && !e.contains("pq")
                && !e.contains("xy")
        })
        .count();
    println!("Part 1: {}", part_one);

    let part_two = input
        .lines()
        .filter(|e| {
            e.as_bytes().windows(2).enumerate().any(|(idx, pat)| {
                let pos = e.find(&String::from_utf8_lossy(pat).into_owned()).unwrap();
                pos != idx && pos + 1 != idx
            }) && e.as_bytes().windows(3).any(|e| e[0] == e[2])
        })
        .count();
    println!("Part 2: {}", part_two);
}
