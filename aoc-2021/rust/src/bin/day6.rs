fn main() {
    let input = include_str!("../../../inputs/day6");
    let mut nums: [[u64; 7]; 2] = input
        .trim()
        .split(',')
        .map(|e| e.parse::<usize>().unwrap())
        .fold([[0; 7]; 2], |mut acc, e| {
            acc[0][e] += 1;
            acc
        });

    for i in 0..256 {
        let new_idx = (i + 2) % 7;
        nums[0][new_idx] += nums[1][new_idx];
        nums[1][new_idx] = nums[0][i % 7];
        if i == 79 || i == 255 {
            println!(
                "Part {}: {}",
                i / 255 + 1,
                nums.iter().flatten().sum::<u64>()
            );
        }
    }
}
