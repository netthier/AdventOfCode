use std::ops::{RangeBounds, RangeInclusive};

fn main() {
    let input = include_str!("../../../inputs/day4");
    let pairs: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> = input
        .lines()
        .map(|e| {
            let mut ranges = e.split(',').map(|e| {
                let mut split = e.split('-').map(|e| e.parse::<u32>().unwrap());
                split.next().unwrap()..=split.next().unwrap()
            });
            (ranges.next().unwrap(), ranges.next().unwrap())
        })
        .collect();

    let mut part_one = 0;
    for (left, right) in pairs.iter() {
        if (left.start() <= right.start() && left.end() >= right.end())
            || (right.start() <= left.start() && right.end() >= left.end())
        {
            part_one += 1;
        }
    }

    println!("Part 1: {part_one}");
}
