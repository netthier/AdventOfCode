fn main() {
    let input = include_str!("../../../inputs/day1");
    let mut sums: Vec<u32> = input
        .split("\n\n")
        .map(|e| e.lines().map(|e| e.parse::<u32>().unwrap()).sum())
        .collect();

    sums.sort_unstable();

    println!("Part 1: {}", sums.last().unwrap());
    println!("Part 2: {}", sums[sums.len() - 3..].iter().sum::<u32>());
}
