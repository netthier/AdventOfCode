fn main() {
    let input = include_str!("../../../inputs/day1");

    let part_1: u32 = input
        .lines()
        .map(|e| {
            let mut num = e.chars().find(|e| e.is_numeric()).unwrap().to_string();
            num.push(e.chars().rev().find(|e| e.is_numeric()).unwrap());
            num.parse::<u32>().unwrap()
        })
        .sum();

    println!("Part 1: {part_1}");

    let part_2: u32 = input
        .lines()
        .map(|e| {
            let mut first = None;
            let mut last = None;
            for i in 0..e.len() {
                let candidate = &e[i..];
                let mut num = candidate.chars().next().unwrap().to_digit(10);
                if num.is_none() {
                    for (idx, num_word) in [
                        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                    ]
                    .iter()
                    .enumerate()
                    {
                        if candidate.starts_with(num_word) {
                            num = Some((idx as u32) + 1);
                            break;
                        }
                    }
                }
                if num.is_some() {
                    if first.is_none() {
                        first = num;
                    }
                    last = num;
                }
            }
            first.unwrap() * 10 + last.unwrap()
        })
        .sum();

    println!("Part 2: {part_2}");
}
