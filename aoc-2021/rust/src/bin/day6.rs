fn main() {
    let input = include_str!("../../../inputs/day6");
    let mut nums: [u128; 7] = input
        .trim()
        .split(',')
        .map(|e| e.parse::<usize>().unwrap())
        .fold([0; 7], |mut acc, e| {
            acc[e] += 1;
            acc
        });

    for i in 0..256 {
        let new_idx = (i + 2) % 7;
        nums[new_idx] += nums[new_idx] >> 64;
        nums[new_idx] &= u128::MAX >> 64;
        nums[new_idx] |= nums[i % 7] << 64;
        if i == 79 || i == 255 {
            println!(
                "Part {}: {}",
                i / 255 + 1,
                nums.iter()
                    .map(|e| (e & u128::MAX >> 64) + (e >> 64))
                    .sum::<u128>()
            );
        }
    }
}
