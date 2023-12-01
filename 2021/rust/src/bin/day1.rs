fn main() {
    let input = include_str!("../../../inputs/day1");
    let nums: Vec<u32> = input.lines().map(|e| e.parse::<u32>().unwrap()).collect();

    let part_one = nums.windows(2).filter(|e| e[1] > e[0]).count();
    println!("Part 1: {}", part_one);

    let part_two = nums.windows(4).filter(|e| e[3] > e[0]).count();
    println!("Part 2: {}", part_two);
}
